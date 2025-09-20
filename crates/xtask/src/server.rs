//! Axum web server for running the webdriver proxy.
//!
//! This proxy server allows the WASM tests to request static assets,
//! as well as report test failures in a (hopefully) nice way.

use axum::{
    body::{Body, Bytes},
    extract::{Path, Request},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, options, post},
    Json, Router,
};
use image::DynamicImage;
use img_diff::DiffCfg;
use tokio::io::AsyncWriteExt;
use wire_types::Error;

pub async fn serve() {
    log::info!("starting the xtask server");
    let app = Router::new()
        .route("/test_img/{*path}", get(static_file))
        .route("/assert_img_eq/{*filename}", options(accept))
        .route("/assert_img_eq/{*filename}", post(assert_img_eq))
        .route("/save/{*filename}", options(accept))
        .route("/save/{*filename}", post(save))
        .route("/artifact/{*filename}", options(accept))
        .route("/artifact/{*filename}", post(artifact))
        .route("/{*rest}", any(accept));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Responds with access control headers to allow anything from anywhere.
async fn accept(request: Request) -> Response {
    log::debug!("accept: {request:#?}");
    Response::builder()
        .status(StatusCode::OK)
        .header("accept", "*/*")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "*")
        .header("access-control-allow-headers", "*")
        .body(Body::default())
        .unwrap()
}

async fn static_file_inner(
    path: impl AsRef<std::path::Path>,
    prefix: impl AsRef<std::path::Path>,
) -> Result<Response, StatusCode> {
    log::info!(
        "requested '{}' '{}'",
        prefix.as_ref().display(),
        path.as_ref().display()
    );
    let mut full_path = prefix.as_ref().join(path);
    if full_path.is_dir() {
        full_path = full_path.join("index.html");
    }
    if full_path.exists() {
        let bytes = tokio::fs::read(&full_path).await.map_err(|e| {
            log::error!("could not read path '{full_path:?}': {e}");
            StatusCode::BAD_REQUEST
        })?;
        let mime = new_mime_guess::from_path(full_path);
        let mimetype = if let Some(mt) = mime.first() {
            mt.to_string()
        } else {
            "application/octet-stream".to_owned()
        };
        let resp = Response::builder()
            .status(StatusCode::OK)
            .header("content-type", mimetype)
            .header("access-control-allow-origin", "*")
            .body(Body::from(Bytes::copy_from_slice(&bytes)))
            .map_err(|e| {
                log::error!("colud not create response: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        Ok(resp)
    } else {
        log::error!("{full_path:?} not found");
        Err(StatusCode::NOT_FOUND)
    }
}

async fn static_file(Path(path): Path<String>) -> Result<Response, StatusCode> {
    let test_img_dir = std::path::PathBuf::from(std::env!("CARGO_WORKSPACE_DIR")).join("test_img");
    static_file_inner(path, test_img_dir).await
}

fn image_from_wire(img: wire_types::Image) -> Result<image::DynamicImage, Error> {
    match img.pixel {
        wire_types::PixelType::Rgb8 => {
            image::RgbImage::from_raw(img.width, img.height, img.bytes).map(DynamicImage::from)
        }
        wire_types::PixelType::Rgba8 => {
            image::RgbaImage::from_raw(img.width, img.height, img.bytes).map(DynamicImage::from)
        }
    }
    .ok_or_else(|| {
        let description = "could not construct image".to_owned();
        log::error!("{description}");
        Error { description }
    })
}

async fn assert_img_eq_inner(
    filename: &str,
    img: wire_types::Image,
) -> Result<(), wire_types::Error> {
    let seen = image_from_wire(img)?;

    img_diff::assert_img_eq_cfg_result(
        filename,
        seen,
        DiffCfg {
            output_dir: img_diff::WASM_TEST_OUTPUT_DIR,
            ..Default::default()
        },
    )
    .map_err(|description| {
        log::error!("{description}");
        Error { description }
    })
}

async fn assert_img_eq(
    headers: HeaderMap,
    Path(parts): Path<Vec<String>>,
    Json(img): Json<wire_types::Image>,
) -> Response {
    let filename = parts.join("/");
    log::info!("asserting '{filename}'");
    log::info!("headers: {headers:#?}");

    let result = assert_img_eq_inner(&filename, img).await;
    Response::builder()
        .status(StatusCode::OK)
        .header("accept", "*/*")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "*")
        .header("access-control-allow-headers", "*")
        .body(Json(result).into_response().into_body())
        .unwrap()
}

async fn save_inner(filename: &str, img: wire_types::Image) -> Result<(), Error> {
    let img = image_from_wire(img)?;
    img_diff::save_to(img_diff::WASM_TEST_OUTPUT_DIR, filename, img)
        .map_err(|description| Error { description })
}

async fn save(
    headers: HeaderMap,
    Path(parts): Path<Vec<String>>,
    Json(img): Json<wire_types::Image>,
) -> Response {
    let filename = parts.join("/");
    log::info!("asserting '{filename}'");
    log::info!("headers: {headers:#?}");
    let result = save_inner(&filename, img).await;
    Response::builder()
        .status(StatusCode::OK)
        .header("accept", "*/*")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "*")
        .header("access-control-allow-headers", "*")
        .body(Json(result).into_response().into_body())
        .unwrap()
}

async fn artifact_inner(filename: impl AsRef<std::path::Path>, body: Body) -> Result<(), Error> {
    use futures_util::StreamExt;

    let mut byte_stream = body.into_data_stream();
    tokio::fs::create_dir_all(
        filename
            .as_ref()
            .parent()
            .ok_or_else(|| Error::from(format!("'{:?}' has no parent dir", filename.as_ref())))?,
    )
    .await
    .map_err(|e| Error::from(e.to_string()))?;
    let mut file = tokio::fs::File::create(filename)
        .await
        .map_err(|e| Error::from(e.to_string()))?;
    while let Some(result_bytes) = byte_stream.next().await {
        let bytes = result_bytes.map_err(|e| Error::from(e.to_string()))?;
        file.write_all(&bytes)
            .await
            .map_err(|e| Error::from(e.to_string()))?;
    }
    Ok(())
}

async fn artifact(Path(parts): Path<Vec<String>>, body: Body) -> Response {
    let filename = std::path::PathBuf::from(img_diff::WASM_TEST_OUTPUT_DIR).join(parts.join("/"));
    log::info!("saving artifact to {filename:?}");
    let result = artifact_inner(filename, body).await;
    Response::builder()
        .status(StatusCode::OK)
        .header("accept", "*/*")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "*")
        .header("access-control-allow-headers", "*")
        .body(Json(result).into_response().into_body())
        .unwrap()
}

pub async fn serve_docs() {
    log::info!("starting the xtask docs server");
    let app = Router::new().route("/{*rest}", get(docs));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    log::info!("serving docs");
    axum::serve(listener, app).await.unwrap();
}

async fn docs(Path(path): Path<String>) -> impl IntoResponse {
    log::info!("path: {path:#?}");
    static_file_inner(path, "target/doc").await
}

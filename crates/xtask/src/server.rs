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
use wire_types::Error;

pub async fn serve() {
    log::info!("starting the webdriver proxy");
    let app = Router::new()
        .route("/test_img/{*path}", get(static_file))
        .route("/assert_img_eq/{*filename}", options(accept))
        .route("/assert_img_eq/{*filename}", post(assert_img_eq))
        .route("/{*rest}", any(accept));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn static_file(Path(path): Path<String>) -> Result<Response, StatusCode> {
    log::info!("requested static '{path}'");
    let test_img = std::path::PathBuf::from(std::env!("CARGO_WORKSPACE_DIR")).join("test_img");
    let path = test_img.join(path);
    if path.exists() {
        let bytes = tokio::fs::read(&path).await.map_err(|e| {
            log::error!("could not read path '{path:?}': {e}");
            StatusCode::BAD_REQUEST
        })?;
        let mime = new_mime_guess::from_path(path);
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
        log::error!("{path:?} not found");
        Err(StatusCode::NOT_FOUND)
    }
}

async fn assert_img_eq_inner(
    filename: &str,
    img: wire_types::Image,
) -> Result<(), wire_types::Error> {
    let seen = match img.pixel {
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
    })?;

    img_diff::assert_img_eq_cfg_result(filename, seen, Default::default()).map_err(|description| {
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

async fn accept(request: Request) -> Response {
    log::info!("accept: {request:#?}");
    Response::builder()
        .status(StatusCode::OK)
        .header("accept", "*/*")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "*")
        .header("access-control-allow-headers", "*")
        .body(Body::default())
        .unwrap()
}

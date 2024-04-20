//! Abstraction over loading bytes on WASM or other.
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum LoadingBytesError {
    #[snafu(display("loading by WASM error: {msg:#?}"))]
    Wasm {
        path: String,
        msg: wasm_bindgen::JsValue,
    },
    #[snafu(display("loading '{path}' from '{}' by filesystem error: {source}", cwd.display()))]
    Fs {
        path: String,
        cwd: std::path::PathBuf,
        source: std::io::Error,
    },
}

/// Load the file at the given url and return it as a vector of bytes, if
// possible.
#[cfg(target_arch = "wasm32")]
pub async fn load(path: &str) -> Result<Vec<u8>, LoadingBytesError> {
    use wasm_bindgen::JsCast;

    let path = path.to_string();
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    let request = web_sys::Request::new_with_str_and_init(&path, &opts).map_err(|msg| {
        LoadingBytesError::Wasm {
            path: path.clone(),
            msg,
        }
    })?;
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|msg| LoadingBytesError::Wasm {
            path: path.clone(),
            msg,
        })?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|msg| LoadingBytesError::Wasm {
            path: path.clone(),
            msg,
        })?;
    let array_promise = resp.array_buffer().map_err(|msg| LoadingBytesError::Wasm {
        path: path.clone(),
        msg,
    })?;
    let buffer = wasm_bindgen_futures::JsFuture::from(array_promise)
        .await
        .map_err(|msg| LoadingBytesError::Wasm {
            path: path.clone(),
            msg,
        })?;
    assert!(buffer.is_instance_of::<js_sys::ArrayBuffer>());
    let array: js_sys::Uint8Array = js_sys::Uint8Array::new(&buffer);
    let mut bytes: Vec<u8> = vec![0; array.length() as usize];
    array.copy_to(&mut bytes);
    Ok(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn load(path: &str) -> Result<Vec<u8>, LoadingBytesError> {
    let bytes: Vec<u8> = async_fs::read(path).await.with_context(|_| FsSnafu {
        path: path.to_string(),
        cwd: std::env::current_dir().unwrap(),
    })?;
    Ok(bytes)
}

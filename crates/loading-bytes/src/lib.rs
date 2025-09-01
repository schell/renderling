//! Abstraction over loading bytes on WASM or other.
use snafu::prelude::*;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Debug, Snafu)]
pub enum WasmError {
    #[snafu(display("Could not create request to load '{path}': {msg:#?}"))]
    CreateRequest {
        path: String,
        msg: send_wrapper::SendWrapper<wasm_bindgen::JsValue>,
    },

    #[snafu(display("Fetch failed to load '{path}' by WASM error: {msg:#?}"))]
    Fetch {
        path: String,
        msg: send_wrapper::SendWrapper<wasm_bindgen::JsValue>,
    },

    #[snafu(display("Fetching '{path}' returned something that was not a Response: {msg:#?}"))]
    NotAResponse {
        path: String,
        msg: send_wrapper::SendWrapper<wasm_bindgen::JsValue>,
    },

    #[snafu(display("Could not get response array buffer '{path}': {msg:#?}"))]
    Array {
        path: String,
        msg: send_wrapper::SendWrapper<wasm_bindgen::JsValue>,
    },

    #[snafu(display("Could not get buffer from array '{path}': {msg:#?}"))]
    Buffer {
        path: String,
        msg: send_wrapper::SendWrapper<wasm_bindgen::JsValue>,
    },

    #[snafu(display("{other}"))]
    Other { other: String },
}

/// Enumeration of all errors this library may result in.
#[derive(Debug, Snafu)]
pub enum LoadingBytesError {
    #[snafu(display("{source}"))]
    Wasm { source: WasmError },

    #[snafu(display("loading '{path}' by filesystem from CWD '{}' error: {source}", cwd.display()))]
    Fs {
        path: String,
        cwd: std::path::PathBuf,
        source: std::io::Error,
    },
}

impl From<WasmError> for LoadingBytesError {
    fn from(value: WasmError) -> Self {
        LoadingBytesError::Wasm { source: value }
    }
}

pub async fn load_wasm(path: &str) -> Result<Vec<u8>, WasmError> {
    use wasm_bindgen::JsCast;

    let path = path.to_string();
    let opts = web_sys::RequestInit::new();
    opts.set_method("GET");
    let request = web_sys::Request::new_with_str_and_init(&path, &opts).map_err(|msg| {
        CreateRequestSnafu {
            path: path.clone(),
            msg: send_wrapper::SendWrapper::new(msg),
        }
        .build()
    })?;
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|msg| {
            FetchSnafu {
                path: path.clone(),
                msg: send_wrapper::SendWrapper::new(msg),
            }
            .build()
        })?;
    let resp: web_sys::Response = resp_value.dyn_into().map_err(|msg| {
        NotAResponseSnafu {
            path: path.clone(),
            msg: send_wrapper::SendWrapper::new(msg),
        }
        .build()
    })?;
    let array_promise = resp.array_buffer().map_err(|msg| {
        ArraySnafu {
            path: path.clone(),
            msg: send_wrapper::SendWrapper::new(msg),
        }
        .build()
    })?;
    let buffer = wasm_bindgen_futures::JsFuture::from(array_promise)
        .await
        .map_err(|msg| {
            BufferSnafu {
                path: path.clone(),
                msg: send_wrapper::SendWrapper::new(msg),
            }
            .build()
        })?;
    assert!(buffer.is_instance_of::<js_sys::ArrayBuffer>());
    let array: js_sys::Uint8Array = js_sys::Uint8Array::new(&buffer);
    let mut bytes: Vec<u8> = vec![0; array.length() as usize];
    array.copy_to(&mut bytes);
    Ok(bytes)
}

pub async fn post_json_wasm<T: serde::de::DeserializeOwned>(
    path: &str,
    data: &str,
) -> Result<T, WasmError> {
    use js_sys::JsString;
    use wasm_bindgen::JsCast;

    let path = path.to_string();
    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    let headers = js_sys::Object::new();
    js_sys::Reflect::set(
        &headers,
        &JsString::from("content-type"),
        &JsString::from("application/json"),
    )
    .unwrap();
    opts.set_headers(&headers);
    let body = js_sys::JsString::from(data);
    opts.set_body(&body.into());
    let request = web_sys::Request::new_with_str_and_init(&path, &opts).map_err(|msg| {
        CreateRequestSnafu {
            path: path.clone(),
            msg: send_wrapper::SendWrapper::new(msg),
        }
        .build()
    })?;
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|msg| {
            FetchSnafu {
                path: path.clone(),
                msg: send_wrapper::SendWrapper::new(msg),
            }
            .build()
        })?;
    let resp: web_sys::Response = resp_value.dyn_into().map_err(|msg| {
        NotAResponseSnafu {
            path: path.clone(),
            msg: send_wrapper::SendWrapper::new(msg),
        }
        .build()
    })?;

    snafu::ensure!(
        resp.ok(),
        OtherSnafu {
            other: wasm_bindgen_futures::JsFuture::from(resp.text().unwrap())
                .await
                .unwrap()
                .as_string()
                .unwrap()
        }
    );

    let value = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap_throw())
        .await
        .unwrap_throw();
    let s = value.as_string().expect_throw(&format!("{value:#?}"));
    let t = serde_json::from_str::<T>(&s).unwrap_throw();
    Ok(t)
}

// TODO: deduplicate post_bin_wasm and post_json_wasm
pub async fn post_bin_wasm<T: serde::de::DeserializeOwned>(
    path: &str,
    data: &[u8],
) -> Result<T, WasmError> {
    use js_sys::JsString;
    use wasm_bindgen::JsCast;

    let path = path.to_string();
    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    let headers = js_sys::Object::new();
    js_sys::Reflect::set(
        &headers,
        &JsString::from("content-type"),
        &JsString::from("application/octet-stream"),
    )
    .unwrap();
    opts.set_headers(&headers);
    let body = js_sys::Uint8Array::from(data);
    opts.set_body(&body.into());
    let request = web_sys::Request::new_with_str_and_init(&path, &opts).map_err(|msg| {
        CreateRequestSnafu {
            path: path.clone(),
            msg: send_wrapper::SendWrapper::new(msg),
        }
        .build()
    })?;
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|msg| {
            FetchSnafu {
                path: path.clone(),
                msg: send_wrapper::SendWrapper::new(msg),
            }
            .build()
        })?;
    let resp: web_sys::Response = resp_value.dyn_into().map_err(|msg| {
        NotAResponseSnafu {
            path: path.clone(),
            msg: send_wrapper::SendWrapper::new(msg),
        }
        .build()
    })?;

    snafu::ensure!(
        resp.ok(),
        OtherSnafu {
            other: wasm_bindgen_futures::JsFuture::from(resp.text().unwrap())
                .await
                .unwrap()
                .as_string()
                .unwrap()
        }
    );

    let value = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap_throw())
        .await
        .unwrap_throw();
    let s = value.as_string().expect_throw(&format!("{value:#?}"));
    let t = serde_json::from_str::<T>(&s).unwrap_throw();
    Ok(t)
}

/// Load the file at the given url fragment or path and return it as a vector of bytes, if
/// possible.
pub async fn load(path: &str) -> Result<Vec<u8>, LoadingBytesError> {
    #[cfg(target_arch = "wasm32")]
    {
        let bytes = load_wasm(path).await?;
        Ok(bytes)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let bytes: Vec<u8> = async_fs::read(path).await.with_context(|_| FsSnafu {
            path: path.to_string(),
            cwd: std::env::current_dir().unwrap(),
        })?;
        Ok(bytes)
    }
}

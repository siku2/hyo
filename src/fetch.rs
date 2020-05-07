use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("{code}: {text}")]
    HttpError { code: u16, text: String },

    #[error("error: {0:?}")]
    Generic(js_sys::Error),

    #[error("unknown error: {0:?}")]
    Unknown(JsValue),
}

impl From<JsValue> for FetchError {
    fn from(v: JsValue) -> Self {
        let err_res = v.dyn_into::<js_sys::Error>();
        match err_res {
            Ok(err) => Self::Generic(err),
            Err(err) => Self::Unknown(err),
        }
    }
}

impl From<&Response> for FetchError {
    fn from(resp: &Response) -> Self {
        Self::HttpError {
            code: resp.status(),
            text: resp.status_text(),
        }
    }
}

pub async fn perform_request(request: Request) -> Result<Response, FetchError> {
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    Ok(resp)
}

pub fn ensure_response_status_ok(resp: &Response) -> Result<(), FetchError> {
    if (200..300).contains(&resp.status()) {
        Ok(())
    } else {
        Err(resp.into())
    }
}

pub async fn perform_text_request(request: Request) -> Result<String, FetchError> {
    let resp = perform_request(request).await?;
    ensure_response_status_ok(&resp)?;
    let text_val = JsFuture::from(resp.text()?).await?;
    Ok(text_val.as_string().unwrap())
}

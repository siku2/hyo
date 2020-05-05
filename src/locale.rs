use fluent::{FluentBundle, FluentResource};
use std::rc::Rc;
use thiserror::Error;
use unic_langid::{langid, LanguageIdentifier};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Error)]
pub enum FetchError {
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

async fn perform_request(request: Request) -> Result<Response, FetchError> {
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    Ok(resp)
}

async fn perform_text_request(request: Request) -> Result<String, FetchError> {
    let resp = perform_request(request).await?;
    let text_val = JsFuture::from(resp.text()?).await?;
    Ok(text_val.as_string().unwrap())
}

async fn fetch_fluent_resource(langid: &LanguageIdentifier) -> Result<FluentResource, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&format!("./locale/{}.ftl", langid), &opts)?;

    // TODO make sure that the response status code is 200
    let raw = perform_text_request(request).await?;
    // TODO error handling
    Ok(FluentResource::try_new(raw).unwrap())
}

struct FluentManager {
    builtin_fallback: Rc<FluentResource>,
    builtin: FluentBundle<Rc<FluentResource>>,
}

impl FluentManager {
    async fn new_load_fallback() -> Result<Self, FetchError> {
        let fallback_langid = langid!("en-GB");
        let resource = Rc::new(fetch_fluent_resource(&fallback_langid).await?);
        let mut bundle = FluentBundle::new(&[fallback_langid]);
        bundle.add_resource(Rc::clone(&resource)).unwrap();

        Ok(Self {
            builtin_fallback: resource,
            builtin: bundle,
        })
    }
}

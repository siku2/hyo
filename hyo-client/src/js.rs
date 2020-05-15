use url::Url;

pub fn get_url() -> Option<Url> {
    web_sys::window()?
        .location()
        .href()
        .ok()
        .and_then(|href| Url::parse(&href).ok())
}

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{ErrorEvent, HtmlElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ImageFallbackProps {
    pub src: String,
    #[prop_or_default]
    pub alt: String,
    pub fallback_src: String,
    #[prop_or_default]
    pub class: String,
}

pub enum ImageFallbackMsg {
    Error,
}

pub struct ImageFallback {
    props: ImageFallbackProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    error_callback: Option<Closure<dyn FnMut(ErrorEvent)>>,
    use_fallback: bool,
}

impl ImageFallback {
    fn view_normal(&self) -> Html {
        let props = &self.props;
        // TODO this is currently hard to implement because yew doesn't recognize the onerror event
        html! {
            <img ref=self.node_ref.clone() class=props.class.clone() src=props.src alt=props.alt/>
        }
    }
    fn view_fallback(&self) -> Html {
        let props = &self.props;
        html! {
            <img ref=self.node_ref.clone() class=props.class.clone() src=props.fallback_src alt=props.alt/>
        }
    }
}

impl Component for ImageFallback {
    type Message = ImageFallbackMsg;
    type Properties = ImageFallbackProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node_ref: NodeRef::default(),
            error_callback: None,
            use_fallback: false,
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if let Some(el) = self.node_ref.cast::<HtmlElement>() {
            let link = self.link.clone();
            let onerror_callback: Closure<dyn FnMut(ErrorEvent)> = Closure::new(move |_| {
                link.send_message(ImageFallbackMsg::Error);
            });
            el.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            self.error_callback = Some(onerror_callback);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use ImageFallbackMsg::*;
        match msg {
            Error => {
                self.use_fallback = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            if self.props.src != props.src {
                self.use_fallback = false;
            }

            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.use_fallback {
            self.view_fallback()
        } else {
            self.view_normal()
        }
    }
}

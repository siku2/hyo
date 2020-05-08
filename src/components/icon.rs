use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct MDIconProps {
    pub icon: String,
}

pub struct MDIcon {
    props: MDIconProps,
}

impl Component for MDIcon {
    type Message = ();
    type Properties = MDIconProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let props = &self.props;

        html! {
            <i class="material-icons">{ props.icon.clone() }</i>
        }
    }
}

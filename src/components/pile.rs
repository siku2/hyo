use super::card::{CardBack, CardFront, CardInfo};
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct CommonPileProps {
    pub children: Children,

    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<()>,
}

struct CommonPile {
    props: CommonPileProps,
}
impl Component for CommonPile {
    type Message = ();
    type Properties = CommonPileProps;

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

        let card_it = props.children.iter().enumerate().map(|(i, child)| {
            html! {
                <div class="pile__item" style=format!("--child-index:{};", i)>
                    { child }
                </div>
            }
        });

        let onclick = props.onclick.clone();
        let onclick = Callback::from(move |_| onclick.emit(()));

        html! {
            <div class="pile" style=format!("--cards:{};", props.children.len()) onclick=onclick>
                { for card_it }
            </div>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct VisiblePileProps {
    pub cards: Vec<CardInfo>,

    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<()>,
}

pub struct VisiblePile {
    props: VisiblePileProps,
}

impl Component for VisiblePile {
    type Message = ();
    type Properties = VisiblePileProps;

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

        let cards: Vec<_> = props
            .cards
            .iter()
            .rev()
            .cloned()
            .map(|info| {
                html! {
                    <CardFront color=info.color number=info.number/>
                }
            })
            .take(3)
            .rev()
            .collect();

        html! {
            <CommonPile onclick=props.onclick.clone()>
                { cards }
            </CommonPile>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct HiddenPileProps {
    pub cards: usize,

    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<()>,
}

pub struct HiddenPile {
    props: HiddenPileProps,
}

impl Component for HiddenPile {
    type Message = ();
    type Properties = HiddenPileProps;

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

        let mut show_count = props.cards;

        const MAX_BEFORE_CUT: usize = 10;
        const AFTER_CUT_SCALE: usize = 3;
        const MAX_TOTAL: usize = 30;

        if show_count > MAX_BEFORE_CUT {
            show_count = MAX_BEFORE_CUT + (show_count - MAX_BEFORE_CUT) / AFTER_CUT_SCALE;
        }

        show_count = show_count.min(MAX_TOTAL);

        let cards: Vec<_> = (0..show_count)
            .map(|_| {
                html! {
                    <CardBack/>
                }
            })
            .collect();

        html! {
            <CommonPile onclick=props.onclick.clone()>
                { cards }
            </CommonPile>
        }
    }
}

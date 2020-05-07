use yew::prelude::*;

#[derive(Clone, Properties)]
struct CircleItemProps {
    pub children: Children,
    pub angle: f32,
}

struct CircleItem {
    props: CircleItemProps,
}

impl Component for CircleItem {
    type Message = ();
    type Properties = CircleItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let props = &self.props;

        let (y_dir, x_dir) = (props.angle - 90.0).to_radians().sin_cos();
        let style = format!(
            "--base-transform:translateX(-50%) rotate({}deg);\
            left:calc(var(--circle-radius) * {});\
            bottom:calc(var(--circle-radius) * {});",
            props.angle, x_dir, -y_dir,
        );
        html! {
            <div style=style>
                { props.children.render() }
            </div>
        }
    }
}

/// Return the smaller of the two values.
/// In all cases where v1 <= v2 holds true, v1 is returned.
fn min_partial<T: std::cmp::PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 {
        v2
    } else {
        v1
    }
}

#[derive(Clone, Properties)]
pub struct CircleProps {
    pub children: Children,
    pub target_angle: f32,
    pub max_total_angle: f32,
}

pub struct Circle {
    props: CircleProps,
}

impl Component for Circle {
    type Message = ();
    type Properties = CircleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let props = &self.props;

        let children = &props.children;
        let balance_count = (children.len() - 1) as f32;
        let max_possible_child_angle = props.max_total_angle / balance_count;
        let child_angle = min_partial(props.target_angle, max_possible_child_angle);
        let start_angle = -(balance_count * child_angle) / 2.0;

        let children_iter = children.iter().enumerate().map(|(i, child)| {
            let angle = start_angle + i as f32 * child_angle;
            html! {
                <CircleItem angle=angle>
                    { child }
                </CircleItem>
            }
        });

        html! {
            <div class="layout-circle">
                { for children_iter }
            </div>
        }
    }
}

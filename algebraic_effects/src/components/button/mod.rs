use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties,Clone, PartialEq)]
pub struct Props {
    pub button_text: String,
    pub onclick: Option<Callback<MouseEvent>>,
}

const STYLE_FILE: &str = include_str!("style.css");

#[styled_component(Button)]
pub fn button(props: &Props) -> Html {
    let style_sheet = Style::new(STYLE_FILE).unwrap();
    let onclick = {
        let props_onclick = props.onclick.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(props_onclick) = props_onclick.clone() {
                props_onclick.emit(event);
            }
        })
    };
    html! {
        <div class={style_sheet}>
        <div class="button" {onclick}>{&props.button_text}</div>
        </div>
    }
}

use crate::router::Route;
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::use_history;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("../styles/home.css");

#[function_component(Home)]
pub fn home() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Rust);
    });
    html! {
          <div class={style}>
          <div class="button" {onclick}>{"start"}</div>
          </div>
        }
}

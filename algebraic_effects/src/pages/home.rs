use crate::router::Route;
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::use_history;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("home.css");

#[function_component(Home)]
pub fn home() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Rust);
    });
    html! {
      <div class={style}>
      <div class="frame">
      <h1 class="title">{"Algebraic_effects"}</h1>
      // <h1 class="title">{"代数效应"}</h1>
      // <h1 class="title-zh">{"代数效应"}</h1>
     
      <div class="button" {onclick}>{"start"}</div>
      <p class="date">{"January 1"}</p>
      <a href="https://github.com/hichho">{"hichho"}</a>
      </div>
      </div>
    }
}

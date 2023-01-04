use crate::components::icon::index::Icon;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("style.css");

#[styled_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    html! {
      <div class={style}>
      <div class="navigation">
      <div class="left-nav">
      <div class="icon-container">
      <Icon height={"3.6vh"} width={"3.6vh"}/>
      </div>
      <p class="title">{"Algebraic Effects"}</p>
      </div>
      // 
      <div class="right-nav">
      <a>{"Stage"}</a>
      <a>{"Speaker"}</a>
      <button>{"123"}</button>
      </div>
      </div>
      </div>
    }
}

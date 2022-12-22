mod pages;
mod router;
use crate::router::{switch, Route};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    html! {
      <div class={style}>
      <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)}/>
      </BrowserRouter>
      </div>
    }
}

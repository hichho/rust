mod components;
mod enum_gather;
mod pages;
mod router;
use crate::components::navigation_bar::index::NavigationBar;
use crate::enum_gather::PaginationEnum;
use crate::router::{switch, Route};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("./styles/main.css");

#[derive(Clone)]
pub struct Pagination {
    pub page: PaginationEnum,
}

#[styled_component(App)]
pub fn app() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    html! {
      <div class={style}>
      <div class="frame">
      <NavigationBar/>
      <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)}/>
      </BrowserRouter>
      </div>
      </div>
    }
}

mod pages;
mod router;
mod components;
mod enum_gather;
use crate::router::{switch, Route};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::enum_gather::PaginationEnum;


const STYLE_FILE: &str = include_str!("main.css");

#[derive(Clone)]
pub struct Pagination{
    pub page:PaginationEnum
}


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

mod components;
mod types;
mod pages;
mod router;
use std::ops::Deref;
use crate::components::navigation_bar::index::NavigationBar;
use crate::router::{switch, Route};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::ContextProvider;
use types::theme::ThemeEnum;
// use gloo::console::log;
use crate::components::icon::index::Icon;

const STYLE_FILE: &str = include_str!("./styles/main.css");
#[derive(PartialEq,Clone)]
pub struct Theme{
  pub theme:ThemeEnum,
}
impl Default for Theme{
  fn default()->Self{
    Self { theme: ThemeEnum::Dark }
  }
}

#[styled_component(App)]
pub fn app() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    let theme_state = use_state(Theme::default);

    // use_effect(||{
    //   log!("");
    // });
    html! {
      <ContextProvider<Theme> context={theme_state.deref().clone()}>
      <div class={style}>
      <div class="frame">
      <NavigationBar/>
      <Icon width={"12vh"} height={"12vh"} animation={true}/>
      <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)}/>
      </BrowserRouter>
      </div>
      </div>
      </ContextProvider<Theme>>
    }
}

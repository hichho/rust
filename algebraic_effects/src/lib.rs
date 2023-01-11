mod components;
mod hooks;
mod pages;
mod router;
mod store;
mod types;
mod util;
use crate::components::icon::Icon;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme::use_theme;
use crate::router::{switch, Route};
use gloo::console::log;
use std::rc::Rc;
use stylist::{yew::styled_component, Style};
use types::theme::ThemeEnum;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;
const STYLE_FILE: &str = include_str!("./styles/main.css");
#[derive(PartialEq, Clone)]
pub struct Theme {
    pub theme: ThemeEnum,
}
impl Default for Theme {
    fn default() -> Self {
        Self { theme: use_theme()}
    }
}
impl Reducible for Theme{
  type Action = ThemeEnum;
  fn reduce(self:Rc<Self>,action:Self::Action)->Rc<Self>{
    Theme {theme:action}.into()
  }
}
pub type ThemeContext = UseReducerHandle<Theme>;
#[styled_component(App)]
pub fn app() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    let theme = use_reducer(|| Theme{
      theme:ThemeEnum::Light
    });
    html! {
      <ContextProvider<ThemeContext> context={theme}>
      <div class={style}>
      <div class="frame">
      <NavigationBar/>
      // <Icon width={"12vh"} height={"12vh"} animation={true}/>
      <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)}/>
      </BrowserRouter>
      </div>
      </div>
      </ContextProvider<ThemeContext>>
    }
}

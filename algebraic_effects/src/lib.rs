mod components;
mod types;
mod pages;
mod router;
use crate::components::navigation_bar::index::NavigationBar;
use crate::router::{switch, Route};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::ContextProvider;
use types::theme::ThemeEnum;
use gloo::console::log;

const STYLE_FILE: &str = include_str!("./styles/main.css");
#[derive(Clone,PartialEq,Default)]
pub struct Theme{
  pub theme:String,
}
// impl Theme{
//   fn default()->ThemeEnum{
//     ThemeEnum::Dark
//   }
// }

#[styled_component(App)]
pub fn app() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    let theme_state = use_state(Theme::default);

    // use_effect(||{
    //   log!("");
    // });
    html! {
      // <ContextProvider<Theme> context={theme_state.deref().clone()}>
      <div class={style}>
      <div class="frame">
      <NavigationBar/>
      <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)}/>
      </BrowserRouter>
      </div>
      </div>
      // </ContextProvider<Theme>>
    }
}

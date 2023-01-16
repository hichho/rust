use crate::{router::Route, ThemeContext};
use yew::prelude::*;
use yew_router::prelude::use_history;
use crate::components::icon::Icon;
use crate::hooks::use_change_theme::use_change_theme;
use yew_router::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/home_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/home_light.css");

#[function_component(Home)]
pub fn home() -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let theme = theme_ctx.theme.to_owned();
    let style = use_change_theme(theme.clone(),DARK_STYLE_FILE,LIGHT_STYLE_FILE);
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Rust);
    });
    html! {
        <div class={style}>
        <div class="home-bg">
        <Icon width={"12vh"} height={"12vh"} animation={true}/>
        <div class="button" {onclick}>{"start"}</div>
        </div>
        </div>
      }
}

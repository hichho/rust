// use crate::hooks::use_style_file_path::use_style_file_path;
use crate::{router::Route};
use yew::prelude::*;
use yew_router::prelude::use_history;
use crate::components::icon::Icon;
use crate::hooks::use_theme_file::use_theme_file;
use yew_router::prelude::*;
use crate::types::icon::IconEnum;

const DARK_STYLE_FILE: &str = include_str!("../styles/home_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/home_light.css");

#[function_component(Home)]
pub fn home() -> Html {
    let style = use_theme_file(DARK_STYLE_FILE,LIGHT_STYLE_FILE);
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Rust);
    });
    html! {
        <div class={style}>
        <div class="home-bg">
        </div>
        <div class="home-container">
        <Icon svg={IconEnum::React} width={"18vh"} height={"18vh"} animation={true}/>
        <Icon svg={IconEnum::Rust} width={"18vh"} height={"18vh"} animation={true}/>
        <div class="button" {onclick}>{"start"}</div>
        </div>
        </div>
      }
}

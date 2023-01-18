// use crate::hooks::use_style_file_path::use_style_file_path;
use crate::{router::Route};
use yew::prelude::*;
use yew_router::prelude::use_history;
use crate::components::icon::Icon;
use crate::hooks::use_theme_file::use_theme_file;
use yew_router::prelude::*;
use crate::types::icon::IconEnum;
use crate::components::navigation_bar::NavigationBar;
// use crate::components::button::Button;
use web_sys::window;

const DARK_STYLE_FILE: &str = include_str!("../styles/home_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/home_light.css");

#[function_component(Home)]
pub fn home() -> Html {
    let style = use_theme_file(DARK_STYLE_FILE,LIGHT_STYLE_FILE);
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Rust);
    });
    let click_react = Callback::from(move|_:_|{
        let url = "https://conf.reactjs.org/";
        let location = window().unwrap().location();
        location.set_href(url).unwrap();
    });
    html! {
        <div class={style}>
        <NavigationBar/>
        <div class="home-bg">
        </div>
        <div class="home-container">
        <div class="icons">
        <div onclick={click_react}>
        <Icon svg={IconEnum::React} width={"18vh"} height={"18vh"} animation={true}/>
        </div>
        <Icon svg={IconEnum::Rust} width={"18vh"} height={"18vh"} animation={true}/>
        </div>
        // <Button>{"开始"}</Button>
        <div class="button" {onclick}>{"start"}</div>
        </div>
        </div>
      }
}

use crate::components::button::Button;
use crate::components::icon::Icon;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use crate::router::Route;
use crate::types::icon::IconEnum;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::use_history;
use yew_router::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/home_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/home_light.css");

#[function_component(Home)]
pub fn home() -> Html {
    let style = use_theme_file(DARK_STYLE_FILE, LIGHT_STYLE_FILE);
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::WebAssembly);
    });
    let click_react = Callback::from(move |_: _| {
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
      <h1 class="home-title">{"Algebraic Effects"}</h1>
      <div class="icons">
      <div onclick={click_react}>
      <Icon svg={IconEnum::React} width={"12vh"} height={"12vh"} animation={true}/>
      </div>
    //   <Icon svg={IconEnum::Rust} width={"12vh"} height={"12vh"} animation={true}/>
      <img src="./assets/rust.png" alt="" style="width:12vh;width:12vh"/>
      </div>
      <Button button_text={"get start !"} {onclick}/>
      </div>
      </div>
    }
}

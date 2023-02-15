use crate::components::menu::Menu;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use web_sys::window;
use yew::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/webassembly_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/webassembly_light.css");
#[function_component(WebAssembly)]
pub fn webassembly() -> Html {
    let style_file = use_theme_file(DARK_STYLE_FILE, LIGHT_STYLE_FILE);
    let click_webassembly = Callback::from(move |_: _| {
        let url = "https://developer.mozilla.org/zh-CN/docs/WebAssembly";
        let location = window().unwrap().location();
        location.set_href(url).unwrap();
    });
    let click_performance = Callback::from(move |_: _| {
        let _url = "https://blog.unity.com/technology/webassembly-load-times-and-performance";
        let _location = window().unwrap().location();
        _location.set_href(_url).unwrap();
    });
    html! {<div class={style_file}>
    <div class="web-frame">
    <NavigationBar/>
    <div class="web-body" onclick={click_webassembly}>
    <Menu/>
    <div class="web-content" onclick={click_performance}>
    <h1 class="title">{"WebAssembly"}</h1>
    <div>
    <h2>{"JavaScript与WebAssembly"}</h2>
    </div>
    </div>
    </div>
    </div>
      </div>
    }
}

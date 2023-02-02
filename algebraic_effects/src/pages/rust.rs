use yew::prelude::*;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;

const DARK_STYLE_FILE: &str = include_str!("../styles/rust_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/rust_light.css");
#[function_component(Rust)]
pub fn rust()->Html{
  let style = use_theme_file(DARK_STYLE_FILE,LIGHT_STYLE_FILE);
  html!{
    <div class={style}>
    <NavigationBar/>
    </div>
  }
}
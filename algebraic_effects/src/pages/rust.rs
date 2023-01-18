use yew::prelude::*;
use crate::components::navigation_bar::NavigationBar;
#[function_component(Rust)]
pub fn rust()->Html{
  html!{
    <h1>
    <NavigationBar/>
    {"rust"}
    </h1>
  }
}
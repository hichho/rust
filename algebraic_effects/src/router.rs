use crate::pages::home::Home;
use crate::pages::rust::Rust;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::webassembly::WebAssembly;
use crate::pages::effect::Effect;
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/webassembly")]
    WebAssembly,
    #[at("/rust")]
    Rust,
    #[at("/effect")]
    Effect
}
pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home/>},
        Route::WebAssembly=> html! {<WebAssembly/>},
        Route::Rust => html!(<Rust/>),
        Route::Effect=>html!(<Effect/>)
    }
}

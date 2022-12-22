use crate::pages::home::Home;
use crate::pages::rust::Rust;
use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/rust")]
    Rust,
}
pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home/>},
        Route::Rust => html!(<Rust/>),
    }
}

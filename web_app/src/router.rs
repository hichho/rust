use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::pages::home::Home;
use crate::components::pages::hello::Hello;
use crate::components::pages::redux::App;
#[derive(Debug,Clone,Routable,PartialEq)]
pub enum Route{
  #[at("/")]
  Home,
  #[at("/hello")]
  Hello,
  #[at("/redux")]
  Redux
}
pub fn switch(route:&Route)->Html{
  match route{
    Route::Home=>html!{<Home/>},
    Route::Hello=>html!{<Hello/>},
    Route::Redux=>html!{<App/>},
  }
}
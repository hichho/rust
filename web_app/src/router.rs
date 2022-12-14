use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::pages::home::Home;
use crate::components::pages::hello::Hello;
use crate::components::pages::redux::ReduxApp;
use crate::components::pages::func_yewdux::FuncYewdux;
#[derive(Debug,Clone,Routable,PartialEq)]
pub enum Route{
  #[at("/")]
  Home,
  #[at("/hello")]
  Hello,
  #[at("/redux")]
  Redux,
  #[at("/func_yewdux")]
  FuncYewdux
}
pub fn switch(route:&Route)->Html{
  match route{
    Route::Home=>html!{<Home/>},
    Route::Hello=>html!{<Hello/>},
    Route::Redux=>html!{<ReduxApp/>},
    Route::FuncYewdux=>html!{<FuncYewdux/>},
  }
}
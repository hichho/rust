use yew::prelude::*;
use crate::router::Route;
use yew_router::prelude::*;
#[function_component(Home)]
pub fn home()->Html{
  html!{

    <div>
    <h1>{"i am home"}</h1>
    <Link<Route> to={Route::Hello}>{"To Hello"}</Link<Route>>
   </div> 
  }
}
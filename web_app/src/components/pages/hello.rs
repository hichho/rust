use yew::prelude::*;
use yew_router::prelude::use_history;
use yew_router::prelude::*;
use crate::router::Route;
#[function_component(Hello)]
pub fn hello()->Html{
  let history=use_history().unwrap();
  let onclick=Callback::from(move|_|{
    history.push(Route::Home);
  });
  html!{
    <div>
    <h1 {onclick}>{"i am hello"}</h1>
    </div>
  }
}
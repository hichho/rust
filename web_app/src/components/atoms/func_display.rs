use yew::prelude::*;
use yewdux::prelude::BasicStore;

use crate::store::func_login::FuncYewduxStore;
use yewdux_functional::use_store;
#[function_component(FuncDisplay)]
pub fn func_display()->Html{
  let store = use_store::<BasicStore<FuncYewduxStore>>();
 let username = store.state().map(|state|state.username.clone()).unwrap_or_default();
 let password= store.state().map(|state|state.password.clone()).unwrap_or_default();
 let token = store.state().map(|state|state.token.clone()).unwrap_or_default();
  html!{
    <div>
    <h1>{"Functional Display Form"}</h1>
    <p>{format!("UserName:{},Password:{}",username,password)}</p>
    <p>{format!("token:{}",token)}</p>
    </div>
  }
}
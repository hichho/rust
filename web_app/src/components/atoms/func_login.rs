use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::store::func_login::FuncYewduxStore;
use yewdux_functional::use_store;

#[function_component(FuncLogin)]
pub fn func_login()->Html{
  let store = use_store::<BasicStore<FuncYewduxStore>>();
  let handle_form_submit=store.dispatch().reduce_callback_with(|state,event:FocusEvent|{
    event.prevent_default();
    let token = "12fdawe232fdas2323rfda".to_owned();
    state.token = token;
  });
  let handle_username_change=store.dispatch().reduce_callback_with(|state,event:Event|{
    let username = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    state.username = username;
  });
  let handle_password_change = store.dispatch().reduce_callback_with(|state,event:Event|{
    let password = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    state.password = password;
  });
  
  html! {
    <form onsubmit={handle_form_submit}>
    <h1>{"Login"}</h1>
    <div>
    <input type="text" placeholder="username" onchange={handle_username_change}/>
    </div>
    <div><input type="password" placeholder="password" onchange={handle_password_change}/>
    </div>
    <div>
    <button>{"Log in"}</button>
    </div>
    </form>
  }
}
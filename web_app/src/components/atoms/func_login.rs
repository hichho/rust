use crate::{store::func_login::FuncYewduxStore};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;
use crate::api::test::test_api;
use gloo::console::log;

#[function_component(FuncLogin)]
pub fn func_login() -> Html {
    let store = use_store::<PersistentStore<FuncYewduxStore>>();
    let handle_form_submit = store
        .dispatch()
        .reduce_callback_with(|state, event: FocusEvent| {
            event.prevent_default();
            //special
            let username = state.username.clone();
            let password = state.password.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let response = test_api(username,password).await;
                log!(response.q2);
            })
        });
    let handle_username_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let username = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.username = username;
        });
    let handle_password_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let password = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.password = password;
        });

    let token = if let Some(state) = store.state() {
        state.token.clone()
    } else {
        String::new()
    };
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
      <p>{format!("token:{}",token)}</p>
      </div>
      </form>
    }
}

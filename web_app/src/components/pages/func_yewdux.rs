use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use yew::prelude::*;

use crate::components::atoms::{func_display::FuncDisplay, func_login::FuncLogin};
#[derive(Clone)]
pub struct User {
    pub token: String,
    pub task: Option<Task>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub code: u32,
    pub message: Option<String>,
    pub total: u32,
}
//array of the respons
// pub struct TaskResponse{
// pub data:Vec<Task>,
// }
#[function_component(FuncYewdux)]
pub fn funcYewdux() -> Html {
    let state = use_state(|| User {
        token: "tokennnnnnnn".to_owned(),
        task: None,
    });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response =
                    Request::get("https://api-build.kepai365.com/app/system/download?type=mac")
                        .header("tokenn", &state.token)
                        .send()
                        .await
                        .unwrap()
                        .json::<Task>()
                        .await
                        .unwrap();
                // log!(serde_json::to_string_pretty(&response).unwrap());
                let mut user = state.deref().clone();
                user.task = Some(response);
                state.set(user)
            })
        })
    };
    html! {
      <div>
      <button onclick={onclick}>{"request"}</button>
      <FuncLogin/>
      <FuncDisplay/>
      <h1>{"func yewdux"}</h1>
      <pre>{serde_json::to_string_pretty(&state.task).unwrap()}</pre>
      </div>
    }
}

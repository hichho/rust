use std::clone;
use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;
#[derive(Default, Clone)]
struct Data {
    pub username: String,
    pub count: u32,
}
#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(|| Data::default());
    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });
    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data);
    });
    // let username_state = use_state(|| "no username set".to_owned());
    // let button_count_state = use_state(|| 0_u32);
    // let cloned_username_state = username_state.clone();
    // let cloned_button_count_state = button_count_state.clone();
    // let username_changed = Callback::from(move |username| {
    //     cloned_username_state.set(username);
    // });
    // let button_clicked = Callback::from(move |_| {
    //     let count = *cloned_button_count_state;
    //     cloned_button_count_state.set(count + 1);
    // });
    html! {
    // <div><TextInput name="username" handle_onchange={username_changed}/>
    // <CustomButton label="Submit" onclick={button_clicked}/>
    // <p>{"Username:"}{&*username_state}</p>
    // <p>{"Button has been clicked"}{*button_count_state}{" times"}</p>
    // </div>
    <div><TextInput name="username" handle_onchange={username_changed}/>
    <CustomButton label="Submit" onclick={button_clicked}/>
    <p>{"Username:"}{&state.username}</p>
    <p>{"Button has been clicked"}{state.count}{" times"}</p>
    </div>
      }
}

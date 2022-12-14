use std::ops::Deref;

use crate::User;
use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;
#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}
#[derive(Properties,PartialEq)]
pub struct Props{
    pub onsubmit:Callback<Data>,
}
#[function_component(CustomForm)]
pub fn custom_form(props:&Props) -> Html {
    let state = use_state(|| Data::default());
    let cloned_state = state.clone();
    let user_context=use_context::<User>();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let language_changed = Callback::from(move |language| {
        let mut data = cloned_state.deref().clone();
        data.favorite_language = language;
        cloned_state.set(data);
    });
    let form_onsubmit=props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move|event:FocusEvent| {
        event.prevent_default();
        let data= cloned_state.deref().clone();
        form_onsubmit.emit(data);
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
    <form onsubmit={onsubmit}>
    <TextInput name="username" handle_onchange={username_changed}/>
    <TextInput name="favorite_language" handle_onchange={language_changed}/>
    <CustomButton label="Submit" />
    <p>{"Username:"}{user_context.clone().unwrap_or_default().username}</p>
    <p>{"Favorite Language:"}{user_context.unwrap_or_default().favorite_language}</p>
    </form>
      }
}

mod components;
mod router;
mod store;
mod api;
use std::ops::Deref;

use crate::router::{switch, Route};
use components::atoms::main_title::{Color, MainTitle};
use components::atoms::struct_hello::StructHello;
use components::molecules::custom_form::CustomForm;
use components::molecules::custom_form::Data;
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;
#[derive(Serialize, Deserialize, Default)]
struct MyObject {
    username: String,
    favorite_language: String,
}
#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}
const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let first_load = use_state(|| true);
    let main_title_load = Callback::from(|message: String| log!(message));
    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.favorite_language = data.favorite_language;
            user_state.set(user);
        })
    };
    let outside_style = Style::new(STYLE_FILE).unwrap();
    let style_sheet = style!(
        r#"
    color:orange;"#
    )
    .unwrap();
    let name: &str = "Brooks";
    let stylesheet = style!(
        r#".title{
    color:red;
   }"#
    )
    .unwrap();
    let my_object: MyObject = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };
    let my_class = "my_title";
    let tasks = vec!["record Video", "grocery shopping", "pet Xilbe"];
    let message: Option<&str> = None;
    log!(name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    use_effect(move || {
        if *first_load {
            first_load.set(false);
        }
        || {}
    });
    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
        <BrowserRouter>
        <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
       <MainTitle title="i am a component" color={Color::Normal} on_load={main_title_load}/>
      <CustomForm onsubmit={custom_form_submit}/>
        <div class={outside_style}>
          <StructHello message={"hello form lib.rs".to_owned()}/>
        <h1 class={my_class}>{"Hello World!!!"}</h1>
        <p>{"Hi!there!"}</p>
        if my_class != "my_class"{
            <p>{"my class"}</p>
        }else{
            <p>{"i am not class"}</p>
        }
        if let Some(message)= message{
            <p>{message}</p>
        }else{
            <p>{"no message to see"}</p>
        }
        <ul>{list_to_html(tasks)}</ul>
        <h1 class={style_sheet}>{"test style_sheet"}</h1>
        <div class={stylesheet}>
        <div class={"title"}>{"hello style"}</div>
        <div class={css!("color:green;font-size:75px;")}>{"hello style"}</div>
        </div>
        </div>
        </ContextProvider<User>>
    }
}
fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}

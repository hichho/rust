use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize)]
struct MyObject{
    username:String,
    favorite_language:String
}

#[function_component(App)]
pub fn app()-> Html{
    let name:&str = "Brooks";
   let my_object:MyObject = MyObject{
    username:name.to_owned(),
    favorite_language:"Rust".to_owned(),
   };
   let my_class = "my_title";
    log!(name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    html!{
        <h1 class={my_class}>{"Hello World!!!"}</h1>
    }
}
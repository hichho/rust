use crate::components::menu::Menu;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use yew::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/rust_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/rust_light.css");
#[derive(PartialEq,Clone)]
struct RustProgrammingLanguage {
    title:String,
    img: String,
}

#[function_component(Rust)]
pub fn rust() -> Html {
    let style = use_theme_file(DARK_STYLE_FILE, LIGHT_STYLE_FILE);
    let rust_programming_language: Vec<RustProgrammingLanguage> = vec![
        RustProgrammingLanguage {
            title: "所有权".to_string(),
            img: "./assets/rust-code-1.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "引用与借用".to_string(),
            img: "./assets/rust.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "流程控制".to_string(),
            img: "./assets/rust.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "条件编译".to_string(),
            img: "./assets/rust.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "隐式返回".to_string(),
            img: "./assets/rust.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "生命周期".to_string(),
            img: "./assets/rust.png".to_string(),
        },
    ];
    html! {<div class={style}>
    <div class="web-frame">
    <NavigationBar/>
    <div class="web-body">
    <Menu/>
    <div class="web-content">
     {list_to_html(rust_programming_language)}
    </div>
    </div>
    </div>
    </div>
    }
}

fn list_to_html(list: Vec<RustProgrammingLanguage>) -> Vec<Html> {
    list.iter()
        .map(|item| {
            html! {
                <div class="card">
                <div class="content">
                <div class="back">
                <div class="back-content">
                <img class="rust-book" src="./assets/rust.jpg" alt=""  />
                 <strong>{item.title.clone()}</strong>
                </div>
                </div>
                <div class="front">
                <div class="img">
                <div class="circle">
                 </div>
                 <div class="circle" id="right">
                 </div>
                 <div class="circle" id="bottom">
                  </div>
                  </div>
                  <div class="front-content">
                  <img src={item.img.clone()} alt=""/>
                  </div>
                </div>
                </div>
                </div>
            }
        })
        .collect()
}

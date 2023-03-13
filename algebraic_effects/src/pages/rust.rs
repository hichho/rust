use crate::components::menu::Menu;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use yew::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/rust_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/rust_light.css");

#[derive(PartialEq, Clone)]
struct RustProgrammingLanguage {
    title: String,
    content: Vec<String>,
    img: String,
}

#[function_component(Rust)]
pub fn rust() -> Html {
    let style = use_theme_file(DARK_STYLE_FILE, LIGHT_STYLE_FILE);
    let rust_programming_language: Vec<RustProgrammingLanguage> = vec![
        RustProgrammingLanguage {
            title: "所有权".to_string(),
            content:vec!["Rust的所有权是用来管理堆内存的，所有权就是堆上数据的拥有和释放权。
            所有权规定了每个值在任何给定时间点只能被一个所有者所拥有，同时该所有者可以使用和释放该值。
            Rust 编译器在编译时检查所有权和借用规则，以保证程序运行时的内存安全性和线程安全性。所有权规则有以下三条:1.Rust中的每个值都有一个变量，称为它的所有者。
            2.一个值在同一时刻有且只有一个所有者。
            3.当所有者超出作用域时，该值会被丢弃。
            ".to_string()],
            img: "./assets/rust-code-1.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "引用与借用".to_string(),
            content:vec!["在Rust中，引用（reference）是一种指向其他值的指针类型，它允许我们在不获取所有权的情况下访问其他变量的值。Rust中的引用使用&符号进行声明，例如&x表示对变量x的引用。".to_string()
            ,"借用（borrowing）是一种特殊类型的引用，它允许我们在不获取所有权的情况下让其他代码访问值。借用使用&符号和代码块的生命周期进行声明，例如&'a mut x表示对可变变量x的借用，并使用生命周期'a来指定该借用的有效期。".to_string(),
            "借用有两种类型：可变借用（mutable borrowing）和不可变借用（immutable borrowing）。可变借用允许我们修改变量的值，而不可变借用只允许我们访问变量的值，不能修改。".to_string(),
            "在Rust中，借用的规则和所有权规则密切相关。一个值只能同时拥有一个所有者，但可以有多个不可变引用或一个可变引用。当一个值被借用时，所有权不会转移，但原始所有者无法对该值进行修改，直到所有借用的引用超出范围为止。这可以避免竞争条件和数据竞争的问题，使得Rust中的并发编程更加安全和可靠。".to_string()
            ],
            img: "./assets/rust-code-2.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "流程控制".to_string(),
            content:vec!["Rust在流程控制方面有一些独特的特点，包括以下几点：".to_string(),
            "匹配模式（match patterns）：Rust的match语句可以匹配多种模式，包括值、范围、通配符等。这使得代码更加灵活和可读，并且可以减少错误的可能性。".to_string(),
            "匹配守卫（match guards）：Rust的match语句还支持匹配守卫，允许在匹配某个模式时添加额外的条件。这可以使得代码更加清晰和简洁。".to_string(),
            "循环标签（loop labels）：Rust的循环语句支持使用标签来标记循环，使得可以在嵌套循环中跳出指定的循环。这可以减少代码的复杂性和错误的可能性。".to_string(),
            "Option和Result类型：在Rust中，Option和Result类型是常见的流程控制类型。Option类型表示一个可能为空的值，Result类型表示一个可能出现错误的结果。这些类型可以强制开发人员显式地处理潜在的错误情况，从而减少程序出错的可能性。".to_string(),
            ],
            img: "./assets/rust-code-3.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "隐式返回".to_string(),
            content:vec!["在Rust中，函数可以使用隐式返回（implicit return）来简化代码。如果函数的最后一个表达式是一个有效的返回值，那么函数将自动返回该值，而无需使用return关键字。".to_string(),
            ],
            img: "./assets/rust-code-4.png".to_string(),
        },
        RustProgrammingLanguage {
            title: "生命周期".to_string(),
            content:vec!["在Rust中，生命周期（lifetime）是一种注解，用于指定变量或引用的作用域。生命周期注解通常写成以撇号（'）开头的标识符，如'a、'b等。在函数定义中，生命周期注解通常用于指定函数参数和返回值的生命周期，以确保代码的正确性和安全性。".to_string(),
            ],
            img: "./assets/rust-code-5.png".to_string(),
        },
    ];
    html! {<div class={style}>
    <div class="web-frame">
    <NavigationBar/>
    <div class="web-body">
    <Menu/>

    <div class="rust-content">
    <h1 class="rust-title">{"Rust"}</h1>
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
                <div class="rust-card">
               <div class="rust-card-title">
               <img class="rust-img" src="./assets/rust.jpg" alt="" />
               <h2 class="rust-card-title-text">{item.title.clone()}</h2>
               </div>

               <div class="rust-content-text">{content_to_html(item.content.clone())}</div>

                <div class="image-frame">
                <img class="rust-code-image" src={item.img.clone()} alt=""/>
                </div>


                </div>
            }
        })
        .collect()
}

fn content_to_html(list: Vec<String>) -> Vec<Html> {
    list.iter()
        .map(|item| {
            html! {
                <div class="content-item">{item}</div>
            }
        })
        .collect()
}

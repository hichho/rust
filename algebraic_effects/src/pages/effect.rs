use crate::components::menu::Menu;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use web_sys::window;
use yew::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/effect_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/effect_light.css");

#[derive(PartialEq, Clone)]
struct EffectPoint {
    title: String,
    content: Vec<String>,
    img:Vec<String>,
}

#[function_component(Effect)]
pub fn effect() -> Html {
    let style = use_theme_file(DARK_STYLE_FILE, LIGHT_STYLE_FILE);
    let click_koka = Callback::from(move |_: _| {
        let url = "https://koka-lang.github.io/koka/doc/index.html";
        let location = window().unwrap().location();
        location.set_href(url).unwrap();
    });
    let effect: Vec<EffectPoint> = vec![
        EffectPoint{
            title: "简介".to_string(),
            content:vec![" koka是一种具有effect(效果类型)和handler(处理程序)的强类型函数式语言。Koka是一种目前正在开发的研究语言。".to_string()],
            img: vec![],
        }, 
        EffectPoint{
            title: "效应与效应处理器".to_string(),
            content:vec!["effect关键字声明一个效应：raise，该效应是一个控制效应，接受msg，表示异常的信息，并且返回类型。safe-divide函数接受两个参数x和y。如果y等于0，则抛出一个效应，信息为div-by-zero。
            safe-divide执行函数8+safe-divide(1,0)，并且他包含一个handler用来处理effect。".to_string()],
            img: vec!["./assets/koka-code-1.png".to_string()]
        }, 
         EffectPoint{
            title: "Resuming".to_string(),
            content:vec!["The power of effect handlers is not just that we can yield to the innermost handler,
             but that we can also resume back to the call site with a result.".to_string()],
            img: vec!["./assets/koka-code-2.png".to_string()]
        },
        EffectPoint{
            title:"Tail-Resumptive Operations".to_string(),
            content:vec![],
            img:vec!["./assets/koka-code-3.png".to_string()]
        },
        EffectPoint{
            title:"Value Operations".to_string(),
            content:vec![],
            img:vec!["./assets/koka-code-4.png".to_string()]
        },
        EffectPoint{
            title:"Abstracting Handlers".to_string(),
            content:vec![
                "As another example, a writer effect is quite common where values are collected by a handler.".to_string(),
                "".to_string(),
            ],
            img:vec!["./assets/koka-code-5.png".to_string()],
        },
        EffectPoint{
            title:"Return Operations".to_string(),
            content:vec![],
            img:vec!["./assets/koka-code-6.png".to_string()]
        },
        EffectPoint{
            title:"State Effect".to_string(),
            content:vec!["Masking Effect,Overriding Handlers,Resuming more than once".to_string()],
            img:vec!["./assets/koka-code-7.png".to_string()],
        },
        EffectPoint{
            title:"effect advantages".to_string(),
            content:vec![
                "是什么让效果处理程序成为良好的控制流抽象？有三个基本优势 关于其他方法：".to_string(),
                "1.效果处理程序可以具有简单的（欣德利-米尔纳）类型。这不像/例如需要 具有答案类型的类型规则（因为 的类型取决于其匹配的上下文）。(reset 用于标记一个计算的开始，并将该计算的结果作为一个值进行返回。在 reset 的作用域内，通过 shift 可以将当前计算的控制流程转移到一个新的上下文中，并传递一个被称为 continuation 的函数作为参数。这个 continuation 函数可以捕获当前计算的上下文信息，并在稍后的某个时候再恢复这个计算的执行。)shiftresetshiftreset".to_string(),
                "2.效果处理程序的作用域由处理程序定义分隔。这就像 / 但不像 .划定恢复的范围具有各种良好的属性，例如高效 实施策略，但也允许模块化组合 （另见奥列格·基谢廖夫的shiftresetcall/cc".to_string(),
                "3.可以自由组合效果器。这与需要 monad 转换器以特定方式组合的一般monad不同。本质上效果处理程序可以自由组合，因为每个效果处理程序最终都可以表示为一个自由 monad的实例，它可以组合。这也意味着某些 monad 不能表示为效果处理程序（即非代数的）。一个特殊的例子是 continuation monad（可以表达call/cc）。".to_string()
            ],
            img:vec!["./assets/koka-code-8.png".to_string(),"./assets/koka-code-9.png".to_string()],
        },
    ];

    html! {<div class={style}>
    <div class="web-frame">
    <NavigationBar/>
    <div class="web-body">
    <Menu/>

    <div class="effect-content">
    <h1 class="effect-title" onclick={click_koka}>{"Effect"}</h1>
     {list_to_html(effect)}
    </div>
    </div>
    </div>
    </div>
    }
}

fn list_to_html(list: Vec<EffectPoint>) -> Vec<Html> {
    list.iter()
        .map(|item| {
            html! {
                <div class="effect-card">
               <div class="effect-card-title">
               <img class="effect-img" src="./assets/koka.png" alt="" />
               <h2 class="effect-card-title-text">{item.title.clone()}</h2>
               </div>

               <div class="effect-content-text">{content_to_html(item.content.clone())}</div>

                <div class="image-frame">
                // <img class="effect-code-image" src={item.img.clone()} alt=""/>
                {image_list(item.img.clone())}
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
fn image_list(list:Vec<String>)->Vec<Html>{
    list.iter().map(|item|{
        html!{
            <img class="effect-code-image" src={item.clone()} alt=""/>
        }
    }).collect()
}

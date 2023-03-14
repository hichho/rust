use crate::components::menu::Menu;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use web_sys::window;
use yew::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/usage_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/usage_light.css");

#[derive(PartialEq, Clone)]
struct UsagePoint {
    title: String,
    content: Vec<String>,
    img:Vec<String>,
}

#[function_component(Usage)]
pub fn usage() -> Html {
    let style = use_theme_file(DARK_STYLE_FILE, LIGHT_STYLE_FILE);
    let  click_react = Callback::from(move |_: _| {
        let url = "https://github.com/facebook/react/blob/main/packages/react-reconciler/src/ReactFiber.js";
        let location = window().unwrap().location();
        location.set_href(url).unwrap();
    });
    let  click_reconciler= Callback::from(move |_: _| {
      let url = "https://twitter.com/asidorenko_/status/1616115618356379663";
      let location = window().unwrap().location();
      location.set_href(url).unwrap();
  });
 


    html! {<div class={style}>
    <div class="web-frame">
    <NavigationBar/>
    <div class="web-body">
    <Menu/>

    <div class="usage-content">
    <h1 class="usage-title">{"Usage"}</h1>


     <div class="usage-card">
     <div class="usage-card-title">
     <img class="usage-img" src="./assets/koka.png" alt="" />
     <h2 class="usage-card-title-text">{"React"}</h2>
     </div>
     <div class="usage-content-text">
     <div class="content-item" style="cursor:pointer;" onclick={click_reconciler}>{"前端框架需要处理复杂情况下的异步可中断更新。"}</div>
     </div>
     <div class="usage-content-text">
     <div class="content-item">{"从React15到React16，协调器（Reconciler）重构的一大目的是：将老的同步更新的架构变为异步可中断更新。

     异步可中断更新可以理解为：更新在执行过程中可能会被打断（浏览器时间分片用尽或有更高优任务插队），当可以继续执行时恢复之前执行的中间状态。
     
     这就是代数效应中try...handle的作用。
     
     其实，浏览器原生就支持类似的实现，这就是Generator。
     
     但是Generator的一些缺陷使React团队放弃了他：
     
     类似async，Generator也是传染性的，使用了Generator则上下文的其他函数也需要作出改变。这样心智负担比较重。
     
     Generator执行的中间状态是上下文关联的。"}</div>
     </div>
      <div class="image-frame">
      <img class="usage-code-image" src="./assets/react.png" alt="" style="cursor:pointer"
      onclick={click_react}
      />
      </div>
      </div>


      <div class="usage-card">
      <div class="usage-card-title">
      <img class="usage-img" src="./assets/koka.png" alt="" />
      <h2 class="usage-card-title-text">{"Hook"}</h2>
      </div>
      <div class="usage-content-text">
      <div class="content-item">{""}</div>
      </div>
       <div class="image-frame">
       <img class="usage-code-image" src="./assets/react-hook.png" alt=""/>
       </div>
       </div>



    </div>
    </div>
    </div>
    </div>
    }
}

use crate::components::menu::Menu;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use web_sys::window;
use yew::prelude::*;

const DARK_STYLE_FILE: &str = include_str!("../styles/webassembly_dark.css");
const LIGHT_STYLE_FILE: &str = include_str!("../styles/webassembly_light.css");
#[function_component(WebAssembly)]
pub fn webassembly() -> Html {
    let style_file = use_theme_file(DARK_STYLE_FILE, LIGHT_STYLE_FILE);
    // todo:pref
    let click_webassembly = Callback::from(move |_: _| {
        let url = "https://developer.mozilla.org/zh-CN/docs/WebAssembly";
        let location = window().unwrap().location();
        location.set_href(url).unwrap();
    });
    let open_performance_article = Callback::from(move |_: _| {
        let _url = "https://blog.unity.com/technology/webassembly-load-times-and-performance";
        let _location = window().unwrap().location();
        _location.set_href(_url).unwrap();
    });

    let open_efficient_article = Callback::from(move |_: _| {
        let _url =
            "https://thenewstack.io/javascript-vs-wasm-which-is-more-energy-efficient-and-faster/";
        let _location = window().unwrap().location();
        _location.set_href(_url).unwrap();
    });
    let open_yew = Callback::from(move |_: _| {
        let _url = "https://yew.rs/";
        let _location = window().unwrap().location();
        _location.set_href(_url).unwrap();
    });
    let open_ambient_article = Callback::from(move |_: _| {
        let _url = "https://www.ambient.run/post/introducing-ambient";
        let _location = window().unwrap().location();
        _location.set_href(_url).unwrap();
    });
    html! {<div class={style_file}>
    <div class="web-frame">
    <NavigationBar/>
    <div class="web-body">
    <Menu/>
    //
    <div class="web-content">
    <h1 class="web-title" onclick={click_webassembly}>{"WebAssembly"}</h1>
    <section>
    <p class="web-introduction">{"WebAssembly 是一种新的编码方式，可以在现代的网络浏览器中运行 － 它是一种低级的类汇编语言，具有紧凑的二进制格式，可以接近原生的性能运行，并为诸如 C / C ++、Rust、Go、Java、C#等语言提供一个编译目标，以便它们可以在浏览器中运行。
    WebAssembly 提供了一条途径，以使得以各种语言编写的代码都可以以接近原生的速度在 Web 中运行。在这种情况下，以前无法以此方式运行的客户端软件都将可以运行在 Web 中。"}</p>
    <div class="section" style="margin-bottom:36px;">
    <h2 class="section-title-normal">{"WebAssembly & JavaScript Advantage"}
    </h2>
    <ul class="section-ul">
    <li><p class="li-p">{"WebAssembly可以提高执行效率，弥补JavaScript在性能上的缺陷。"}</p></li>
    <li><p class="li-p">{"WebAseembl可以利用现在的底层语言和工具链，让开发者更容易地将原生应用移植到Web平台。"}</p></li>
    <li><p class="li-p">{"WebAseembl可以与JavaScript互操作，共享内存和对象，并且不需要额外的开销。"}</p></li>
    </ul>
    <div class="link-pool">
     <button class="cssbuttons-io-button" style="width:142px;" onclick={open_performance_article}> {"performance"}
    <div class="icon">
    <svg height="24" width="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M0 0h24v24H0z" fill="none"></path><path d="M16.172 11l-5.364-5.364 1.414-1.414L20 12l-7.778 7.778-1.414-1.414L16.172 13H4v-2z" fill="currentColor"></path></svg>
     </div>
    </button>
    <button class="cssbuttons-io-button" style="width:110px;" onclick={open_efficient_article}> {"efficient"}
    <div class="icon">
    <svg height="24" width="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M0 0h24v24H0z" fill="none"></path><path d="M16.172 11l-5.364-5.364 1.414-1.414L20 12l-7.778 7.778-1.414-1.414L16.172 13H4v-2z" fill="currentColor"></path></svg>
     </div>
    </button>
      <button class="cssbuttons-io-button" style="width:110px;" onclick={open_ambient_article}> {"ambient"}
    <div class="icon">
    <svg height="24" width="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M0 0h24v24H0z" fill="none"></path><path d="M16.172 11l-5.364-5.364 1.414-1.414L20 12l-7.778 7.778-1.414-1.414L16.172 13H4v-2z" fill="currentColor"></path></svg>
     </div>
    </button>
    </div>
    </div>
    <div class="section">
    <h2 class="section-title-normal">{"WebAssembly & JavaScript Disadvantage"}</h2>
    <ul class="section-ul">
    <li><p class="li-p">{"WebAssembly目前还不支持垃圾回收（GC），因此需要手动管理内存。"}</p></li>
    <li><p class="li-p">{"WebAssembly目前还不支持多线程（Multhreading），因此不能充分利用多核处理器。"}</p></li>
    <li><p class="li-p">{"WebAssembly目前还不支持直接访问DOM，因此需要通过JavaScript来实现界面交互。"}</p></li>
    </ul>
      <div class="link-pool">
     <button class="cssbuttons-io-button" style="width:74px;" onclick={open_yew}> {"yew"}
    <div class="icon">
    <svg height="24" width="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M0 0h24v24H0z" fill="none"></path><path d="M16.172 11l-5.364-5.364 1.414-1.414L20 12l-7.778 7.778-1.414-1.414L16.172 13H4v-2z" fill="currentColor"></path></svg>
     </div>
    </button>
    </div>
    </div>
    </section>
    </div>
    //
    </div>
    </div>
      </div>
    }
}

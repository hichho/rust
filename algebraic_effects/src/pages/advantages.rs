use crate::components::menu::Menu;
use crate::components::navigation_bar::NavigationBar;
use crate::hooks::use_theme_file::use_theme_file;
use yew::prelude::*;


const DARK_STYLE_FILE:&str = include_str!("../styles/advantages_dark.css");
const LIGHT_STYLE_FILE:&str = include_str!("../styles/advantages_light.css");

#[derive(PartialEq,Clone)]
struct AdvantagesPoint{
  title:String,
  img:Vec<String>,
}

#[function_component(Advantages)]
pub fn advantages()->Html{
  let style = use_theme_file(DARK_STYLE_FILE,LIGHT_STYLE_FILE);
  let advantages_point:Vec<AdvantagesPoint> = vec![
    AdvantagesPoint{
      title:"有‘颜色’的函数".to_string(),
      img:vec!["./assets/advantages-code-1.png".to_string()],
    },
    AdvantagesPoint{
      title:"分离代码中的how与what".to_string(),
      img:vec!["./assets/advantages-code-2.png".to_string()],
    }
  ];
  html!{
    <div class={style}>
    <div class="web-frame">
    <NavigationBar/>
    <div class="web-body">
    <Menu/>

    <div class="advantages-content">
    <h1 class="advantages-title">{"Advantages"}</h1>
    {list_to_html(advantages_point)}
    </div>
    </div>
    </div>
    </div>
  }
}


fn list_to_html(list: Vec<AdvantagesPoint>) -> Vec<Html> {
  list.iter()
      .map(|item| {
          html! {
              <div class="advantages-card">
             <div class="advantages-card-title">
             <img class="advantages-img" src="./assets/koka.png" alt="" />
             <h2 class="advantages-card-title-text">{item.title.clone()}</h2>
             </div>


              <div class="image-frame">
              {image_list(item.img.clone())}
              </div>
              </div>
          }
      })
      .collect()
}
fn image_list(list:Vec<String>)->Vec<Html>{
  list.iter().map(|item|{
      html!{
          <img class="advantages-code-image" src={item.clone()} alt=""/>
      }
  }).collect()
}

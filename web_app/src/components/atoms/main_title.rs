use stylist::{yew::styled_component, style,Style};
use yew::prelude::*;
#[derive(Properties,PartialEq)]
pub struct Props{
// pub title:Option<String>,
pub title:String,
pub color:Color,
pub on_load:Callback<String>
}

impl Color{
  pub fn to_string(&self)->String{
    match self{
      Color::Normal=>"normal".to_owned(),
      Color::Ok=>"ok".to_owned(),
      Color::Error=>"error".to_owned(),
    }
  }
}
#[derive(PartialEq)]
pub enum Color{
  Normal,
  Ok,
  Error
}
#[styled_component(MainTitle)]
pub fn main_title(props:&Props)->Html{
  props.on_load.emit("i loaded!!!".to_owned());
  let style_sheet:Style = style!(r#"
  .normal{
    color:white;
  }
  .ok{
    color:green;
  }
  .error{
    color:red;
  }
  "#).unwrap();
  html!{
    <div class={style_sheet}>
    <h1 class={props.color.to_string()}>{&props.title}</h1>
    </div>
  }
}
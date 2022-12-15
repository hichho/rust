use yew::prelude::*;
use stylist::{ style,Style};
#[derive(Properties,PartialEq)]
pub struct Props{
  pub message:String,
}
pub struct StructHello{
  pub stylesheet:Style,
}
impl StructHello {
  fn style()->Style{
    style!(r#"
    color:green!important;
    "#).unwrap()
  }
}
impl Component for StructHello{
  type Message = ();
  type Properties = Props;
  fn create(_ctx: &Context<Self>) -> Self {
      Self { 
      stylesheet: Self::style() 
    }
  }
  fn update(&mut self,_ctx:&Context<Self>,_msg:Self::Message)->bool{
    false
  }
  fn changed(&mut self, _ctx: &Context<Self>) -> bool {
      true
  }
  
  fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
      
  }

  fn destroy(&mut self, _ctx: &Context<Self>) {
      
  }
  
  
  fn view(&self,ctx: &Context<Self>) -> Html {
      html!{
       <h1 class={self.stylesheet.clone()}>{&ctx.props().message}</h1> 
      }
  }
}
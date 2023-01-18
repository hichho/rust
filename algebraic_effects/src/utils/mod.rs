use web_sys::window;
use crate::types::router::RouterEnum;
pub fn router_go(router:RouterEnum)->(){
  let mut url = "";
  match router {
    RouterEnum::Home=>url = "http://127.0.0.1:8080",
    RouterEnum::Rust=>(),
  }
  if url!=""{
    let location = window().unwrap().location();
    location.set_href(url).unwrap();
  }

}
use yew::prelude::*;

use crate::components::atoms::{func_display::FuncDisplay, func_login::FuncLogin};

#[function_component(FuncYewdux)]
pub fn funcYewdux()-> Html{
  html!{
    <div>
    <FuncLogin/>
    <FuncDisplay/>
    <h1>{"func yewdux"}</h1>
    </div>
  }
}
use std::fmt::Display;

use yew::prelude::*;
use yewdux::{prelude::*, dispatch};
use crate::components::atoms::counter::Counter;
use crate::components::atoms::display::DisplayCount;
use crate::store::counter::{init,YewduxStore};

pub struct App{
    dispatch:Dispatch<BasicStore<YewduxStore>>
}
impl Component for App {
    type Message = ();
    type Properties = DispatchProps<BasicStore<YewduxStore>>;
    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = init();
        Self { dispatch }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div><h1>{"App"}</h1>
          <WithDispatch<Counter> />
          <WithDispatch<DisplayCount> />
          </div>
        }
    }
}

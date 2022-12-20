use yew::prelude::*;
use yewdux::{prelude::*};

use crate::store::login::YewduxStore;

pub struct Display{
    _dispatch:DispatchProps<BasicStore<YewduxStore>>
}

impl Component for Display {
    type Message = ();
    type Properties = DispatchProps<BasicStore<YewduxStore>>;
    fn create(ctx: &Context<Self>) -> Self {
        let _dispatch = ctx.props().dispatch().clone();
        Self { _dispatch }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let username = &ctx.props().state().username;
        let password= &ctx.props().state().password;
        html! {
        <div>
        <h1>{"Display Form"}</h1>
        <p>{format!("Username:{},Password:{}",username,password)}</p>
        </div>
       }
    }
}

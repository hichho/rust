
use yew::prelude::*;
use yewdux::{prelude::*, dispatch};
use crate::components::atoms::login::Login;
use crate::components::atoms::display::Display;
use crate::store::login::{init,YewduxStore};

//redux 必须用在router之外
pub struct ReduxApp{
    dispatch:Dispatch<BasicStore<YewduxStore>>
}
impl Component for ReduxApp{
    type Message = ();
    type Properties = DispatchProps<BasicStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        let dispatch = init();
        Self { dispatch }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div>
            <h1>{"App"}</h1>
            <WithDispatch<Login> />
            <WithDispatch<Display> />
          </div>
        }
    }
}

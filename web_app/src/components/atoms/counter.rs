use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::counter::YewduxStore;
pub struct Counter;
impl Component for Counter {
    type Message = ();
    type Properties = DispatchProps<BasicStore<YewduxStore>>;
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;
        let onclick = ctx.props().dispatch().reduce_callback(|state| state.count += 1);
        html! {
          <div>
          <h1>{"Counter"}</h1>
          <p>{format!("The button has been pressed {} times",count)}</p>
          <button onclick={onclick}>{"try to click me!"}</button>
          </div>
        }
    }
}

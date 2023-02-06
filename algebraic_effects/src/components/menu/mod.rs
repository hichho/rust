use crate::{
    store::{menu::{MenuStore}, self},
    types::menu::{MenuItemEnum, MENU_ARRAY},
};
use crate::types::menu::MenuTab;
use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::use_store;
const STYLE_FILE: &str = include_str!("style.css");
#[styled_component(Menu)]
pub fn menu() -> Html {
    let store = use_store::<PersistentStore<MenuStore>>();
    let style = Style::new(STYLE_FILE).unwrap();
    let handle_click_menu = store
        .dispatch()
        .reduce_callback_with(|state, value: MenuTab| {
            state.menu_tab = value;
        });
  
        

    html! {
          <div class={style}>
          <div class="menu-frame">
          <div class="menu-title-container">
          <div><input/></div>


    <div class="menu-container">
        <div class="menu-item">
          <div class="menu-title">{"Menu"}</div>
          <div class="menu-border"></div>
        </div>
        <div class="menu-item">
          <div class="menu-title">{"Other"}</div>
          <div class="menu-border"></div>
        </div>
    </div>


          </div>
          {menu_enum_to_html(MENU_ARRAY)}
          </div>
          </div>
        }
}
fn menu_enum_to_html(menu: [MenuItemEnum; 2]) -> Vec<Html> {
    menu.iter()
        .map(|item| {
            html! {
              <div class="menu-text">
              <span>{item}</span>
              <svg transform="rotate(-90)" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" class="duration-100 ease-in transition -rotate-90" style="min-width:20px;min-height:20px"><g fill="none" fill-rule="evenodd" transform="translate(-446 -398)"><path fill="currentColor" fill-rule="nonzero" d="M95.8838835,240.366117 C95.3957281,239.877961 94.6042719,239.877961 94.1161165,240.366117 C93.6279612,240.854272 93.6279612,241.645728 94.1161165,242.133883 L98.6161165,246.633883 C99.1042719,247.122039 99.8957281,247.122039 100.383883,246.633883 L104.883883,242.133883 C105.372039,241.645728 105.372039,240.854272 104.883883,240.366117 C104.395728,239.877961 103.604272,239.877961 103.116117,240.366117 L99.5,243.982233 L95.8838835,240.366117 Z" transform="translate(356.5 164.5)"></path><polygon points="446 418 466 418 466 398 446 398"></polygon></g></svg>
              </div>
            }
        })
        .collect()
}

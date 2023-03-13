use crate::router::Route;
use crate::store::menu::{change_menu_tab, set_current_menu};
use crate::types::menu::MenuTab;
use crate::{
    store::menu::MenuStore,
    types::menu::{MenuItemEnum, MENU_ARRAY},
};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::use_history;
use yew_router::prelude::*;
use yewdux::prelude::*;
const STYLE_FILE: &str = include_str!("style.css");

#[styled_component(Menu)]
pub fn menu() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    let (_store, store_dispatch) = use_store::<MenuStore>();

    let handle_click_menu = {
        let store_dispatch = store_dispatch.clone();
        Callback::from(move |_| {
            let store_dispatch = store_dispatch.clone();
            change_menu_tab(MenuTab::MenuLabel, store_dispatch);
        })
    };
    let handle_click_other = {
        let store_dispatch = store_dispatch.clone();
        Callback::from(move |_| {
            let store_dispatch = store_dispatch.clone();
            change_menu_tab(MenuTab::OtherLabel, store_dispatch);
        })
    };

    // let menu_tab = _store.menu_tab.clone();

    html! {
          <div class={style}>
          <div class="menu-frame">
          <div class="menu-title-container">
        //   <div><input/></div>
    <div class="menu-container">
        <div  onclick={handle_click_menu}>
          <div class={get_selected_menu_class(MenuTab::MenuLabel)}>{"Menu"}</div>
          <div class={get_selected_menu_border_class(MenuTab::MenuLabel)}></div>
        </div>
        <div onclick={handle_click_other}>
          <div class={get_selected_menu_class(MenuTab::OtherLabel)}>{"Other"}</div>
          <div class={get_selected_menu_border_class(MenuTab::OtherLabel)}></div>
        </div>
    </div>
          </div>
          {menu_enum_to_html(&MENU_ARRAY)}
          </div>
          </div>
        }
}
fn menu_enum_to_html(menu: &'static [MenuItemEnum; 3]) -> Vec<Html> {
    let history = use_history().unwrap();
    let (_store, store_dispatch) = use_store::<MenuStore>();
    let handle_change_menu = {
        let store_dispatch = store_dispatch.clone();
        Callback::from(move |menu_item: MenuItemEnum| {
            history.push(Route::WebAssembly);
            match menu_item {
                MenuItemEnum::WebAssembly => {
                    history.push(Route::WebAssembly);
                }
                MenuItemEnum::Rust => {
                    history.push(Route::Rust);
                }
                MenuItemEnum::Effect=> {
                    history.push(Route::Effect);
                }
            }
            let store_dispatch = store_dispatch.clone();
            set_current_menu(menu_item, store_dispatch);
        })
    };
    menu.iter()
        .map(|menu_item| {
                let onclick = handle_change_menu.reform(move|_| menu_item.clone());
            html! {
              <div class={get_selected_menu_item(*menu_item)} onclick={onclick}>
              <span>{menu_item}</span>
              <svg transform="rotate(-90)" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" class="duration-100 ease-in transition -rotate-90" style="min-width:20px;min-height:20px"><g fill="none" fill-rule="evenodd" transform="translate(-446 -398)"><path fill="currentColor" fill-rule="nonzero" d="M95.8838835,240.366117 C95.3957281,239.877961 94.6042719,239.877961 94.1161165,240.366117 C93.6279612,240.854272 93.6279612,241.645728 94.1161165,242.133883 L98.6161165,246.633883 C99.1042719,247.122039 99.8957281,247.122039 100.383883,246.633883 L104.883883,242.133883 C105.372039,241.645728 105.372039,240.854272 104.883883,240.366117 C104.395728,239.877961 103.604272,239.877961 103.116117,240.366117 L99.5,243.982233 L95.8838835,240.366117 Z" transform="translate(356.5 164.5)"></path><polygon points="446 418 466 418 466 398 446 398"></polygon></g></svg>
              </div>
            }
        })
        .collect()
}

fn get_selected_menu_class(menu_tab: MenuTab) -> String {
    let mut menu_class = "menu-title-unchecked".to_string();
    let (_store, _store_dispatch) = use_store::<MenuStore>();
    if _store.menu_tab == menu_tab {
        menu_class = "menu-title".to_string();
    }
    menu_class
}

fn get_selected_menu_border_class(menu_tab: MenuTab) -> String {
    let mut menu_border_class = "menu-border-unchecked".to_string();
    let (_store, _store_dispatch) = use_store::<MenuStore>();
    if _store.menu_tab == menu_tab {
        menu_border_class = "menu-border".to_string();
    }
    menu_border_class
}

fn get_selected_menu_item(menu: MenuItemEnum) -> String {
    let mut menu_item_class = "menu-text-unchecked".to_string();
    let (_store, _store_dispatch) = use_store::<MenuStore>();
    if _store.current_menu == menu {
        menu_item_class = "menu-text".to_string();
    }
    menu_item_class
}

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use crate::types::menu::MenuTab;
use crate::types::menu::MenuItemEnum;

#[derive(Clone, Serialize, Deserialize,Store,PartialEq)]
#[store(storage="local")]
pub struct MenuStore {
    pub menu_tab: MenuTab,
    pub current_menu: MenuItemEnum,
}
impl Default for MenuStore {
    fn default() -> Self {
        Self {
            menu_tab: MenuTab::MenuLabel,
            current_menu: MenuItemEnum::WebAssembly,
        }
    }
}
pub fn change_menu_tab(menu_tab:MenuTab,dispatch:Dispatch<MenuStore>){
    dispatch.reduce(move|store|{
        let mut store = store.deref().clone();
        store.menu_tab = menu_tab;
        store
    });
}
use crate::types::menu::MenuItemEnum;
use crate::types::menu::MenuTab;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use yewdux::prelude::*;

#[derive(Clone, Serialize, Deserialize, Store, PartialEq)]
#[store(storage = "local")]
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
pub fn change_menu_tab(menu_tab: MenuTab, dispatch: Dispatch<MenuStore>) {
    dispatch.reduce(move |store| {
        let mut store = store.deref().clone();
        store.menu_tab = menu_tab;
        store
    });
}
pub fn set_current_menu(current_menu: MenuItemEnum, dispatch: Dispatch<MenuStore>) {
    dispatch.reduce(move |store| {
        let mut store = store.deref().clone();
        store.current_menu = current_menu;
        store
    })
}

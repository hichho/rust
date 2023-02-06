use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use crate::types::menu::MenuTab;
use crate::types::menu::MenuItemEnum;


#[derive(Clone, Serialize, Deserialize)]
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
impl Persistent for MenuStore{
    fn key() -> &'static str {
        ""
    }
    fn area() -> Area {
        Area::Local
    }
}

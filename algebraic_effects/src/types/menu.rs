use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub enum MenuItemEnum {
    WebAssembly,
    Rust,
}
impl fmt::Display for MenuItemEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MenuItemEnum::WebAssembly => write!(f, "WebAssembly"),
            MenuItemEnum::Rust => write!(f, "Rust"),
        }
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum MenuTab {
    MenuLabel,
    OtherLabel,
}
impl fmt::Display for MenuTab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MenuTab::MenuLabel => write!(f, "Menu"),
            MenuTab::OtherLabel => write!(f, "other"),
        }
    }
}
pub const MENU_ARRAY: [MenuItemEnum; 2] = [MenuItemEnum::WebAssembly, MenuItemEnum::Rust];

use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub enum MenuItemEnum {
    WebAssembly,
    Rust,
    Effect
}
impl fmt::Display for MenuItemEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MenuItemEnum::WebAssembly => write!(f, "WebAssembly"),
            MenuItemEnum::Rust => write!(f, "Rust"),
            MenuItemEnum::Effect=> write!(f, "Effect"),
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
            MenuTab::MenuLabel => write!(f, "menu"),
            MenuTab::OtherLabel => write!(f, "other"),
        }
    }
}
impl Default for MenuTab {
    fn default()->Self{
        MenuTab::MenuLabel
    }
}
pub const MENU_ARRAY: [MenuItemEnum; 3] = [MenuItemEnum::WebAssembly, MenuItemEnum::Rust,MenuItemEnum::Effect];

use std::fmt;
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum MenuEnum {
    WebAssembly,
    Rust,
}
impl fmt::Display for MenuEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MenuEnum::WebAssembly => write!(f, "WebAssembly"),
            MenuEnum::Rust => write!(f, "Rust"),
        }
    }
}
pub const MENUARRAY: [MenuEnum; 2] = [MenuEnum::WebAssembly, MenuEnum::Rust];

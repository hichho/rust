use crate::{
    types::{icon::IconEnum, theme::ThemeEnum},
    ThemeContext,
};
use yew::use_context;
#[derive(PartialEq,Debug)]
pub struct IconColor {
    react_dark_color: String,
    react_light_color: String,
    rust_dark_color: String,
    rust_light_color: String,
}

pub fn use_icon_color(icon: IconEnum) -> String {
    let icon_color = IconColor {
        react_dark_color: "#58c4dc".to_string(),
        react_light_color: "#087ea4".to_string(),
        rust_dark_color: "rgba(250,250,250,1)".to_string(),
        rust_light_color: "black".to_string(),
    };
    // let icon_color = IconColor::default();
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let theme = theme_ctx.theme.to_owned();

    let icon_color = match icon {
        IconEnum::React => match theme {
            ThemeEnum::Dark => icon_color.react_dark_color,
            ThemeEnum::Light => icon_color.react_light_color,
        },
        IconEnum::Rust => match theme {
            ThemeEnum::Dark => icon_color.rust_dark_color,
            ThemeEnum::Light => icon_color.rust_light_color,
        },
    };
    icon_color
}

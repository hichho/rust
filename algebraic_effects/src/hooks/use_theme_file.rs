use yew::prelude::*;
use stylist::{Style};

use crate::{types::theme::ThemeEnum,ThemeContext};
pub fn use_theme_file(dark_style_path:&str,light_style_path:&str)->Style {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let theme = theme_ctx.theme.to_owned();
    match theme{
      ThemeEnum::Dark=>Style::new(dark_style_path).unwrap(),
      ThemeEnum::Light=>Style::new(light_style_path).unwrap(),
    }
}

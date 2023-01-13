use crate::types::theme::ThemeEnum;
use stylist::{Style};

pub fn use_change_theme(theme:ThemeEnum,dark_style_path:&str,light_style_path:&str)->Style{
  match theme{
    ThemeEnum::Dark=>Style::new(dark_style_path).unwrap(),
    ThemeEnum::Light=>Style::new(light_style_path).unwrap(),
  }
}
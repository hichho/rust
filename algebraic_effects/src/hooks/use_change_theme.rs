use crate::types::theme::ThemeEnum;
use stylist::{Style};

pub fn use_change_theme(theme:ThemeEnum,dark_style:Style,light_style:Style)->Style{
  match theme{
    ThemeEnum::Dark=>dark_style,
    ThemeEnum::Light=>light_style,
  }
}
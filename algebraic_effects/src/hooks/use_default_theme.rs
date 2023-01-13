use crate::types::theme::ThemeEnum;
use js_sys::Date;
pub fn use_default_theme()-> ThemeEnum{
  let hour = Date::get_hours(&Date::new_0());
  if hour <=6 || hour>=18{
    ThemeEnum::Dark
  } else{
    ThemeEnum::Light
  }
}
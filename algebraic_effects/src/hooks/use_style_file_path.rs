use crate::types::theme::ThemeEnum;
pub fn _use_style_file_path(theme: ThemeEnum, pre_file_path: Option<&str>) -> String {
    let mut _style_file_path = String::new();
    match theme {
        ThemeEnum::Dark => _style_file_path = _add_pre_file_path("dark_theme.css",pre_file_path),
        ThemeEnum::Light => _style_file_path = _add_pre_file_path("light_theme.css", pre_file_path),
    }
    _style_file_path
}

fn _add_pre_file_path(path: &str, pre_str: Option<&str>) ->String{
    let mut _finally_path = String::new();
    match pre_str {
        Some(i) => _finally_path = i.to_string()+&path.to_string(),
        None => _finally_path = path.to_string(),
    }
    _finally_path
}

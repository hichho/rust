use crate::{components::icon::Icon, ThemeContext};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use crate::types::theme::ThemeEnum;
const STYLE_FILE: &str = include_str!("style.css");

#[styled_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let style = Style::new(STYLE_FILE).unwrap();
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let theme = theme_ctx.theme.to_owned();
    let onclick = Callback::from(move|_| 
      {
        let mut expected_theme = ThemeEnum::Dark;
        match theme_ctx.theme{
          ThemeEnum::Dark=>{
            expected_theme = ThemeEnum::Light;
          },
          ThemeEnum::Light=>(),
        }
       theme_ctx.dispatch(expected_theme);
      }
      );
    html! {
      <div class={style}>
      <div class="navigation">
      <div class="left-nav">
      <div class="icon-container">
      <Icon height={"3.6vh"} width={"3.6vh"}/>
      </div>
      <p class="title">{"Algebraic Effects"}</p>
      </div>
      <div class="right-nav">
      <a>{"Stage"}</a>
      <a>{"Speaker"}</a>
      <button {onclick}>{theme}</button>
      </div>
      </div>
      </div>
    }
}

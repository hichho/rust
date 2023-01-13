use std::fmt;
#[derive(Clone,PartialEq,Debug,Copy)]
pub enum ThemeEnum{
  Dark,Light
}
impl fmt::Display for ThemeEnum{
  fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
    match *self{
      ThemeEnum::Dark=>write!(f,"dark"),
      ThemeEnum::Light=>write!(f,"light"),
    }
  }
}
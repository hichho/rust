use serde::{Serialize, Deserialize};
use yewdux::prelude::*;

#[derive(Clone,Default,Serialize,Deserialize)]
pub struct FuncYewduxStore{
    pub username:String,
    pub password:String,
    pub token:String,
}
impl Persistent for FuncYewduxStore{
    fn key()->&'static str{
        "IntroductionToYew.rs"
    }
    fn area()->Area{
        Area::Local
    }
}
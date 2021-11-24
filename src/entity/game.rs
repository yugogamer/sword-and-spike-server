use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use crate::service::game::Player;



#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CurrentPlayerList{
    pub data : Vec<Player>
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Connection{
    pub name : String
}

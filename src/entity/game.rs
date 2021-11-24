use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use crate::service::game::{Case, MAP_HEIGHT, MAP_WIDTH, Player, Direction};



#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CurrentPlayerList{
    pub data : Vec<Player>
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Connection{
    pub name : String
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Map{
    pub map : [[Case; MAP_WIDTH]; MAP_HEIGHT],
}

pub struct Move{
    pub direction : Direction,
}
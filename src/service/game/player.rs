use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::map::{Direction, Position};

const BASE_HP : i32 = 3;
pub const PLAYER_DOMMAGE: u32 = 1;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Attack{
    pub player_id : u32,
    pub direction : Direction,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Move{
    pub player_id : u32,
    pub direction : Direction,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub enum PlayerState{
    Alive,
    Dead,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Player{
    pub id: u32,
    pub name : String,
    pub hp : i32,
    pub pos : Position,
    pub state : PlayerState
}

impl Player {
    
    pub fn new(name : String, pos : Position) -> Player{
        Player{
            id : 0,
            name : name,
            pos : pos,
            hp : BASE_HP,
            state : PlayerState::Alive
        }
    }

    pub fn take_dommage(&mut self, dommage : i32){
        self.hp -= dommage;
        if self.hp <= 0 {
            self.state = PlayerState::Dead;
        }
    }
    
}
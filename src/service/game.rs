use serde::{Deserialize, Serialize};


const BASE_HP : u32 = 3;

#[derive(Debug, Deserialize, Serialize)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}


#[derive(Debug, Deserialize, Serialize)]
pub struct  Position{
    pub x : u32,
    pub y : u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player{
    pub id: u32,
    pub name : String,
    pub hp : u32,
    pub pos : Position
}

impl Player {
    
    pub fn new(id : u32, name : String, pos : Position) -> Player{
        Player{
            id : id,
            name : name,
            pos : pos,
            hp : BASE_HP
        }
    }
}

pub struct Case{

}
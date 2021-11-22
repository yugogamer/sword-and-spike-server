use serde::{Deserialize, Serialize};


const BASE_HP : u32 = 3;
const MAP_HEIGHT : usize = 25;
const MAP_WIDTH : usize = 25;

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

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Case{
    Wall,
    Pick,
    Air
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attack{
    player_id : u32,
    direction : Direction,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Move{
    player_id : u32,
    direction : Direction,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Action{
    Attack(Attack),
    Move(Move)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Map{
    array : [[Case ; MAP_WIDTH] ; MAP_HEIGHT ],
    players :  Vec<Player>,
    attack_pile: Vec<Attack>,
    move_pile : Vec<Move>
}


fn move_player(current_player : &mut Player, array : &[[Case ; MAP_WIDTH] ; MAP_HEIGHT ], new_pos : Position){
    match array[new_pos.x as usize][new_pos.y as usize] {
        Wall => {},
        Air => {current_player.pos = new_pos},
        Pick => {current_player.pos = new_pos},
    }
}


impl Map {
    
    fn new() -> Map{
        let array : [[Case ; MAP_WIDTH] ; MAP_HEIGHT ] = [ [Case::Air; MAP_WIDTH] ; MAP_HEIGHT];


        Map{
            array : array,
            players : Vec::new(),
            attack_pile : Vec::new(),
            move_pile : Vec::new()
        }
    }

    async fn run(&self){
        let mut already_play : Vec<u32> = Vec::new();

        for p in self.attack_pile.into_iter(){

        }

        for m in self.move_pile.into_iter(){
            let mut player = &self.players[m.player_id as usize];
            match m.direction{
                Up => { move_player(player, &self.array, Position{x : player.pos.x, y : player.pos.y + 1}) },
                Down => { move_player(player, &self.array, Position{x : player.pos.x, y : player.pos.y - 1}) },
            }
        }

    }

}
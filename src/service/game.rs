use serde::{Deserialize, Serialize};


const BASE_HP : u32 = 3;
const MAP_HEIGHT : usize = 25;
const MAP_WIDTH : usize = 25;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}


#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Attack{
    player_id : u32,
    direction : Direction,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
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
        Case::Wall => {},
        Case::Air => {current_player.pos = new_pos},
        Case::Pick => {current_player.pos = new_pos},
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

    fn add_player(&mut self, player : Player){
        self.players.push(player);
    }

    fn adding_attack(&mut self, attack : Attack){
        let already_attack = self.attack_pile.iter().find(|id|id.player_id == attack.player_id);
        match already_attack{
            Some(_) => {},
            None => {
                self.attack_pile.push(attack);
            }
        }
    }

    fn adding_move(&mut self, movement : Move){
        let already_move = self.move_pile.iter().find(|id|id.player_id == movement.player_id);
        match already_move{
            Some(_) => {},
            None => {
                self.move_pile.push(movement);
            }
        }
    }
    
    fn run(&mut self){
        let mut already_play : Vec<u32> = Vec::new();
        
        for p in self.attack_pile.iter(){
            
        }
        
        for m in self.move_pile.iter(){
            if !already_play.contains(&m.player_id){
                let player: &mut Player = self.players.get_mut(m.player_id as usize).unwrap();
                match m.direction{
                    Direction::Up => { move_player(player, &self.array, Position{x : player.pos.x, y : player.pos.y + 1}) },
                    Direction::Down => { move_player(player, &self.array, Position{x : player.pos.x, y : player.pos.y - 1}) },
                    Direction::Left => { move_player(player, &self.array, Position{x : player.pos.x - 1, y : player.pos.y}) },
                    Direction::Right => { move_player(player, &self.array, Position{x : player.pos.x + 1, y : player.pos.y}) },
                }
                println!("{}", serde_json::to_string(&player).unwrap());
                already_play.push(m.player_id)
            }
        }

        self.attack_pile.clear();
        self.move_pile.clear();
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;


    #[test_case(1,  2, Direction::Up  ; "Up test")]
    #[test_case(1,  0, Direction::Down  ; "Down test")]
    #[test_case(0,  1, Direction::Left  ; "Left test")]
    #[test_case(2,  1, Direction::Right  ; "Right test")]
    fn player_movement(x : u32, y : u32, direction : Direction) {
        let expected = Position{x : x, y : y};

        let mut game = Map::new();
        game.add_player(Player::new(0, "yugo".to_string(), Position { x: 1, y: 1 }));
        game.adding_move(Move{
            player_id: 0,
            direction: direction
        });
        game.run();

        assert_eq!(game.players[0].pos.y,expected.y)
    }

    fn player_attack(){
        
    }
}
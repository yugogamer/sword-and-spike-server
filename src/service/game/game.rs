use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};


#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;


    #[test_case(1,  2, Direction::Up  ; "Up test")]
    #[test_case(1,  0, Direction::Down  ; "Down test")]
    #[test_case(0,  1, Direction::Left  ; "Left test")]
    #[test_case(2,  1, Direction::Right  ; "Right test")]
    fn player_movement(x : i32, y : i32, direction : Direction) {
        let expected = Position{x : x, y : y};

        let mut game = Map::new();
        game.add_player(Player::new("yugo".to_string(), Position { x: 1, y: 1 }));
        game.adding_move(Move{
            player_id: 0,
            direction: direction
        });
        game.run();
        
        let valid = expected.x == game.players[0].pos.x && expected.y == game.players[0].pos.y;

        assert_eq!(valid ,true)
    }

    #[test_case(1,  1, Direction::Up  ; "Up test")]
    fn player_movement_player_on_top(x : i32, y : i32, direction : Direction) {
        let expected = Position{x : x, y : y};

        let mut game = Map::new();
        game.add_player(Player::new("yugo".to_string(), Position { x: 1, y: 1 }));
        game.add_player(Player::new("yugo".to_string(), Position { x: 1, y: 2 }));
        game.adding_move(Move{
            player_id: 0,
            direction: direction
        });
        game.run();

        assert_eq!(game.players[0].pos.y,expected.y)
    }

    #[test_case(Direction::Up, 2  ; "Up test")]
    fn player_attack(direction : Direction, exepected : i32){

        let mut game = Map::new();
        game.add_player(Player::new("yugo".to_string(), Position { x: 1, y: 1 }));
        game.add_player(Player::new("yugo".to_string(), Position { x: 1, y: 2 }));
        game.adding_attack( Attack{
            player_id: 0,
            direction: direction
        });
        game.run();

        assert_eq!(game.players[1].hp ,exepected)

    }
}
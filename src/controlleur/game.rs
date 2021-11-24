use crate::{entity::game::{Connection, CurrentPlayerList, Map}, service::game::{Direction, Player}};
//use crate::service::authentification::generate_id;
use rocket::{State, serde::{json::Json}};
use rocket_okapi::{openapi, openapi_get_routes};
use crate::Game;
use crate::Position;
use crate::ServicePlayer;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    return loader.mount("/game/content", openapi_get_routes![settings: join_game, get_map, get_player_list, move_player]);
}

/// # add a player to the game
#[openapi]
#[post("/join", format = "json", data = "<player>")]
async fn join_game(game : &State<Game>, players : &State<ServicePlayer>, player : Json<Connection>) -> Json<Player> {
    let data = player.into_inner();
    let arc = &game.inner().game;
    let current_game = &mut *arc.lock().unwrap();

    current_game.add_player(Player::new(data.name, Position { x: 0, y: 0 }));
    let players = &players.inner().players;
    let players = current_game.players.clone();
    let player_list = Clone::clone(players.last().unwrap());
    drop(current_game);
    drop(arc);

    Json(player_list)
}


/// # get_player_list
#[openapi]
#[get("/player_list")]
async fn get_player_list(game : &State<Game>) -> Json<CurrentPlayerList> {
    let arc = &game.inner().game;
    let current_game = &mut *arc.lock().unwrap();

    let player_list = current_game.players.clone();
    drop(current_game);
    drop(arc);

    Json(CurrentPlayerList{data : player_list})
}

/// # get the current map
#[openapi]
#[get("/get_map")]
async fn get_map(game : &State<Game>) -> Json<Map> {
    let arc = &game.inner().game;
    let current_game = &mut *arc.lock().unwrap();

    let map = current_game.array.clone();
    drop(current_game);
    drop(arc);

    Json(Map{map : map})
}

/// # move the current players
#[openapi]
#[post("/move", format = "json", data = "<movement>")]
async fn move_player(game : &State<Game>, movement : Json<Direction> ) {
    let arc = &game.inner().game;
    let current_game = &mut *arc.lock().unwrap();

    
    drop(current_game);
    drop(arc);
}
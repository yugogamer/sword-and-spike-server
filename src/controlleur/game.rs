use crate::{entity::game::{Connection, CurrentPlayerList, Map}, service::game::{Player}};
//use crate::service::authentification::generate_id;
use rocket::{State, serde::{json::Json}};
use rocket_okapi::{openapi, openapi_get_routes};
use crate::Game;
use crate::Position;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    return loader.mount("/game/content", openapi_get_routes![settings: join_game, get_map]);
}

/// # add a player to the game
#[openapi]
#[post("/join", format = "json", data = "<player>")]
async fn join_game(game : &State<Game>, player : Json<Connection>) -> Json<CurrentPlayerList> {
    let data = player.into_inner();
    let arc = &game.inner().game;
    let current_game = &mut *arc.lock().unwrap();

    current_game.add_player(Player::new(0, data.name, Position { x: 0, y: 0 }));
    let player_list = current_game.players.clone();
    drop(current_game);
    drop(arc);

    Json(CurrentPlayerList{data : player_list})
}

/// # add a player to the game
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
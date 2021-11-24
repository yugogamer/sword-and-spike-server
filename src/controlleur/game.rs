use crate::{entity::game::{CurrentPlayerList, Connection}, service::game::{self, Player}};
//use crate::service::authentification::generate_id;
use rocket::{State, serde::{json::Json}};
use rocket_okapi::{openapi, openapi_get_routes};
use crate::Game;
use crate::Position;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    return loader.mount("/game/content", openapi_get_routes![settings: join_game]);
}

/// # add a player to the game
#[openapi]
#[post("/join", format = "json", data = "<player>")]
async fn join_game(game : &State<Game>, player : Json<Connection>) -> Json<CurrentPlayerList> {
    let data = player.into_inner();
    let arc = &game.inner().game;
    let current_game = &mut *arc.lock().unwrap();

    current_game.add_player(Player::new(0, data.name, Position { x: 0, y: 0 }));

    Json(CurrentPlayerList{data : current_game.players.clone()})
}
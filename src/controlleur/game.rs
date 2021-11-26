use crate::{entity::game::{Connection, CurrentPlayerList, Map, MoveInfo}, service::game::{map::Position, player::{Attack, Move, Player}}};
use rocket::{State, http::Cookie, serde::{json::Json}};
use rocket_okapi::{openapi, openapi_get_routes};
use crate::{service::id_verification::SessionId};
use rocket::http::CookieJar;
use crate::Game;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    return loader.mount("/game/content", openapi_get_routes![settings: join_game, get_map, get_player_list, move_player, attack_player, run, get_current_player]);
}

/// # add a player to the game
#[openapi]
#[post("/join", format = "json", data = "<player>")]
async fn join_game(game : &State<Game>, player : Json<Connection> , cookies: &CookieJar<'_>) -> Json<Player> {
    let mut map = game.inner().game.write().unwrap();
    let player_name = player;

    let new_player = Player::new(player_name.name.clone(), Position { x: 0, y: 0 });
    let response_player = new_player.clone();
    map.add_player(new_player);
    drop(map);


    let session_id = response_player.id;
    let cookie = Cookie::build("session-id", session_id.to_string())
    .path("/")
    .secure(true)
    .finish();


    cookies.add(cookie);
    return Json(response_player);
}


/// # get_player_list
#[openapi]
#[get("/player_list")]
async fn get_player_list(game : &State<Game>) -> Json<CurrentPlayerList> {
    let current_game = game.inner().game.read().unwrap();

    let player_list = current_game.players.clone();
    drop(current_game);

    Json(CurrentPlayerList{data : player_list})
}

/// # get_player_list
#[openapi]
#[get("/player")]
async fn get_current_player(game : &State<Game>, id: SessionId) -> Json<Player> {
    let current_game = game.inner().game.read().unwrap();

    let player = current_game.players.iter().find(|player| player.id == id.id);

    match player {
        Some(player) => return Json(player.clone()),
        None => return Json(Player::new("error".to_string(), Position{x : 0, y : 0})),
    }
}

/// # get the current map
#[openapi]
#[get("/get_map")]
async fn get_map(game : &State<Game>) -> Json<Map> {
    let current_game = game.inner().game.read().unwrap();

    let map = current_game.array.clone();
    drop(current_game);

    Json(Map{map : map})
}

/// # move the current players
#[openapi]
#[post("/move", format = "json", data = "<movement>")]
async fn move_player(game : &State<Game>, id: SessionId ,movement : Json<MoveInfo> ) {
    let mut current_game = game.inner().game.write().unwrap();
    let current_move = Move{ player_id: id.id, direction: movement.direction };

    current_game.adding_move(current_move);
}

/// # define the next attech
#[openapi]
#[post("/attack", format = "json", data = "<movement>")]
async fn attack_player(game : &State<Game>, id: SessionId ,movement : Json<MoveInfo> ) {
    let mut current_game = game.inner().game.write().unwrap();
    let current_attack = Attack{ player_id: id.id, direction: movement.direction };

    current_game.adding_attack(current_attack);
}

/// # run a tick
#[openapi]
#[get("/run")]
async fn run(game : &State<Game>) {
    let mut current_game = game.inner().game.write().unwrap();
    current_game.run();
}
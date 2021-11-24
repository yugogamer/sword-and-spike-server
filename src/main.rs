#[macro_use] extern crate rocket;
use rocket_okapi::{swagger_ui::*};
use crate::service::game::{Player, Position};
use std::sync::{Arc, Mutex};

mod service;
mod controlleur;
mod entity;

struct Game {
    game : Arc<Mutex<service::game::Map>>
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let game = Game {game : Arc::from(Mutex::new(service::game::Map::new()))};

    let loader = rocket::build();
    let loader = controlleur::game::load_road(loader).manage(game);

    let loader = loader.mount(
        "/doc/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/game/content/openapi.json".to_owned(),
            ..Default::default()
        }),
    );


    loader.launch().await
}

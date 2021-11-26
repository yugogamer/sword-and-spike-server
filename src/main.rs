#[macro_use] extern crate rocket;
use rocket_okapi::{swagger_ui::*};
use rocket::Request;
use service::game::map::Map;
use std::sync::RwLock;

mod service;
mod controlleur;
mod entity;


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}


struct Game {
    pub game : RwLock<Map>
}




#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let game = Game {game : RwLock::new(Map::new())};

    let loader = rocket::build();
    let loader = controlleur::game::load_road(loader).manage(game).register("/",catchers![not_found]);

    let loader = loader.mount(
        "/doc/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/game/content/openapi.json".to_owned(),
            ..Default::default()
        }),
    );


    loader.launch().await
}

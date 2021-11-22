#[macro_use] extern crate rocket;
use rocket_okapi::{swagger_ui::*};
use crate::service::game::{Player, Position};

mod service;
mod controlleur;
mod entity;




#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build();


    let pos = Position{x : 1, y : 1};
    let player = Player::new(1, "Hehe".into(), pos);

    println!("{}", serde_json::to_string(&player).unwrap());



    loader.launch().await
}

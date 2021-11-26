#[macro_use] extern crate rocket;
use rocket_okapi::{swagger_ui::*};
use rocket::{Request};
use service::game::map::Map;
use core::time;
use std::{sync::{Arc, RwLock}, thread::{self}, time::SystemTime};


mod service;
mod controlleur;
mod entity;


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}


struct Game {
    pub game : Arc<RwLock<Map>>
}

fn make_tick(lock_game : Arc<RwLock<Map>>){
    println!("beginning game calculation");
    let tick_time = 50;
    let mut tick_date: SystemTime = SystemTime::now();
    let mut tick: u128 = 0;
    let mut action_size = 0;
    loop {
        let time_elapsed = tick_date.elapsed().unwrap();
        if time_elapsed.as_millis() < tick_time
        {
            let sleep_time = (tick_time - time_elapsed.as_millis() as u128).try_into().unwrap();
            std::thread::sleep(time::Duration::from_millis(sleep_time));
            println!("tick : {} execute in : {} ms with : {} action, sleep time : {}", tick, time_elapsed.as_millis(), action_size, sleep_time);
        }else {
            println!("tick : {} execute in : {} ms with : {} action, sleep time : {}", tick, time_elapsed.as_millis(), action_size, 0);
        }
        tick+=1;
        tick_date = SystemTime::now();
        let current_game = &mut *lock_game.write().unwrap();
        action_size = current_game.attack_pile.len() + current_game.move_pile.len();
        current_game.run();
        drop(current_game);
    }
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let lock = RwLock::new(Map::new());
    let duplicate = Arc::new(lock);
    let game = Game {game : Arc::clone(&duplicate)};


    let loader = rocket::build();
    let loader = controlleur::game::load_road(loader).manage(game).register("/",catchers![not_found]);

    let loader = loader.mount(
        "/doc/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/game/content/openapi.json".to_owned(),
            ..Default::default()
        }),
    );

    thread::spawn(move || {
        make_tick(duplicate);
    });



    loader.launch().await
}

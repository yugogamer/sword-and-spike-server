use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};
use rocket::State;
use rocket_okapi::okapi::openapi3::{Object, Parameter, ParameterValue};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use std::error::Error;

use crate::Game;

use super::game::Player;


pub struct SessionId(u32);



#[derive(Debug)]
pub enum SessionIdError{
    BadCout,
    Missing,
    Invalid,
}

fn is_valid_id(id : &u32, players : &Vec<Player>) -> bool{
    let player = players.iter().find(|player_id| *id == player_id.id);
    match player{
        Some(_) => return true,
        None => return false,
    }
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionId{
    type Error = SessionIdError;
    
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookie = req.cookies().get("session-id").map(|c| format!("{}", c.value()));
        let current_user_list = &*req.guard::<&State<Game>>().await.unwrap().inner().game.read().unwrap();
        let current_players = current_user_list.players.clone();
        drop(current_user_list);
        
        if cookie.is_some(){
            let value = cookie.unwrap().parse::<u32>();
            match value {
                Ok(id) => {
                    if is_valid_id(&id, &current_players){
                        return Outcome::Success(SessionId(id));
                    }else {
                        return Outcome::Failure((Status::Forbidden, SessionIdError::Invalid));
                    }
                },
                Err(_) => return Outcome::Failure((Status::Forbidden, SessionIdError::BadCout)),
            }
        }else{
            return Outcome::Failure((Status::NotAcceptable ,SessionIdError::Missing));
        }
    }
}


impl<'r> OpenApiFromRequest<'r> for SessionId{
   

    fn from_request_input(
        gen: &mut rocket_okapi::gen::OpenApiGenerator,
        name: String,
        required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let schema = gen.json_schema::<String>();
        Ok(RequestHeaderInput::Parameter(Parameter {
            name: "session-id".to_owned(),
            location: "cookie".to_owned(),
            description: Some(String::from("Current player id")),
            required,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: false,
                schema,
                example: None,
                examples: None,
            },
            extensions: Object::default(),
        }))
    }

    fn get_responses(_gen: &mut rocket_okapi::gen::OpenApiGenerator) -> rocket_okapi::Result<rocket_okapi::okapi::openapi3::Responses> {
        Ok(rocket_okapi::okapi::openapi3::Responses::default())
    }
}
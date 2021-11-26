use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};
use rocket::State;
use crate::entity::user::ClientConnection;
use crate::service::service_user;
use crate::service::database::Database;
use tokio_postgres::Client;
use sha256::digest;
use std::error::Error;

pub struct SessionId(u32);



#[derive(Debug)]
pub enum SessionIdError{
    BadCout,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionId{
    type Error = SessionIdError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session_id : Vec<_> = req.headers().get("session-id").collect();
        let cookie = req.cookies().get("session-id").map(|c| format!("{}", c.value()));

        if cookie.is_some(){
            let value = cookie.unwrap();
            if is_valid_id(&value[..]) {
                return Outcome::Success(SessionId(value));
            }else {
                return Outcome::Failure((Status::new(403), SessionIdError::Invalid));
            }
        }

        match session_id.len(){
            0 => Outcome::Failure((Status::BadRequest, SessionIdError::Missing)),
            1 if is_valid_id(session_id[0]) => Outcome::Success(SessionId(session_id[0].to_string())),
            1 => Outcome::Failure((Status::new(403), SessionIdError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, SessionIdError::BadCout)),
        }
    }
}
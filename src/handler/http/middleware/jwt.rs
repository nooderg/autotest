use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use crate::core::services::user;
use std::io::{Error, ErrorKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenClaims {
    pub sub: String,
    pub exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserTokenClaims {
    type Error = String;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<Self, String> {

        match get_token_from_header(request) {
            Ok(t) => match user::verify_token(&t) {
                Ok(t) => return Outcome::Success(t.claims),
                Err(e) => return Outcome::Failure((
                    Status::Unauthorized,
                    e.to_string(),
                ))
            }
            Err(e) => return Outcome::Failure((
                Status::Unauthorized,    
                e.to_string()
            ))
        };
    }
}

fn get_token_from_header(request: &Request) -> Result<String, Error> {
    if let Some(authen_header) = request.headers().get_one("Authorization") {
        let authen_str = authen_header.to_string();
        if authen_str.starts_with("Bearer") {
            return Ok(authen_str[6..authen_str.len()].trim().to_owned());
        } 
    }
    return Err(Error::from(ErrorKind::InvalidData));
}


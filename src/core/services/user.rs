use pwhash::bcrypt;
use jsonwebtoken::{encode, decode, errors, Header, EncodingKey, DecodingKey, Algorithm, Validation, TokenData};
use std::io::{Error, ErrorKind};
use chrono::Utc;

use crate::handler::http::middleware;


pub fn hash_password(password: String) -> String {
    bcrypt::hash(password).unwrap()
}


static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

pub fn create_token(id: uuid::Uuid) -> Result<String, Error>{
    let key = b"secret";
    let my_claims = middleware::jwt::UserTokenClaims{
        sub: id.to_string(),
        exp: 0,
    };

    match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => return Ok(t),
        Err(err) => return Err(Error::new(ErrorKind::BrokenPipe, err)),
    };
}

pub fn verify_token(token: &str) -> Result<TokenData<middleware::jwt::UserTokenClaims>, Error> {
    let validation = Validation::new(Algorithm::HS256);
    
    match decode::<middleware::jwt::UserTokenClaims>(&token, &DecodingKey::from_secret(b"secret"), &validation) {
        Ok(c) => return Ok(c),
        Err(err) => match *err.kind() {
            errors::ErrorKind::InvalidToken => return Err(Error::new(ErrorKind::InvalidInput, err)),
            errors::ErrorKind::InvalidIssuer => return Err(Error::new(ErrorKind::InvalidInput, err)),
            _ => return Err(Error::new(ErrorKind::BrokenPipe, err)),
        }
    };
}

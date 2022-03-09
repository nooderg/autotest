use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::error::Error;

use rocket::http::Method;
use rocket::{get, routes};



pub fn cors() -> rocket_cors::CorsOptions {
    let allowed_origins = AllowedOrigins::All;

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Patch, Method::Delete, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
}

use actix_web::{web, App, HttpServer, Responder};
use bson::oid::ObjectId;
use actix_web::{error, http::header, http::StatusCode, HttpResponse, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};

use serde::Serialize;
#[path = "./models.rs"]
mod models;

#[derive(Debug, Serialize)]
pub struct Error {
    msg: String,
    aux: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct PostResult {
    pub err_code: i32
}

impl PostResult {
    pub fn new() -> PostResult {
        PostResult {
            err_code: 0
        }
    }
}

impl ResponseError for Error {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> web::HttpResponse {
        let err_json = json!({ "msg": self.msg, "aux": self.aux });
        web::HttpResponse::build(StatusCode::from_u16(400).unwrap())
            .json(err_json)
    }
}

pub async fn submit_character(body: actix_web::web::Json<models::CharacterSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_character for token={}", body.vote_token);
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

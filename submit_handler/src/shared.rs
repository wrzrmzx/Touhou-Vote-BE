
use actix_web::{web};
use actix_web::{http::StatusCode, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct Error {
    msg: String,
    aux: String
}

impl Error {
    pub fn new(msg2: &str) -> Error {
        Error {
            msg: msg2.into(),
            aux: "".into()
        }
    }
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

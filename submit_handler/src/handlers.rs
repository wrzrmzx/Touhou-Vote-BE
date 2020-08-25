
use actix_web::{web, App, HttpServer, Responder};
use bson::oid::ObjectId;
use actix_web::{error, http::header, http::StatusCode, HttpResponse, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};

use serde::Serialize;

use crate::models;
use crate::shared::*;

type SubmitServiceWrapper = web::Data<crate::services::SubmitService>;

pub async fn submit_character(service: SubmitServiceWrapper, body: actix_web::web::Json<models::CharacterSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_character for token={}", body.vote_token);
    service.submitCharcater(body.0).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_music(service: SubmitServiceWrapper, body: actix_web::web::Json<models::MusicSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_music for token={}", body.vote_token);
    service.submitMusic(body.0).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_cp(service: SubmitServiceWrapper, body: actix_web::web::Json<models::CPSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_cp for token={}", body.vote_token);
    service.submitCP(body.0).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_work(service: SubmitServiceWrapper, body: actix_web::web::Json<models::WorkSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_work for token={}", body.vote_token);
    service.submitWork(body.0).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_paper(service: SubmitServiceWrapper, body: actix_web::web::Json<models::PaperSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_paper for token={}", body.vote_token);
    service.submitPaper(body.0).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

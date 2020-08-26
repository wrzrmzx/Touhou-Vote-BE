
use actix_web::{web, App, HttpServer, Responder};
use bson::oid::ObjectId;
use actix_web::{error, http::header, http::StatusCode, HttpResponse, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::{json, to_string_pretty};

use serde::Serialize;

use crate::models;
use crate::shared::*;

pub const SUBMIT_VALIDATOR: &'static str = "127.0.0.1:1103";

type SubmitServiceV1Wrapper = web::Data<crate::services::SubmitServiceV1>;

pub async fn submit_character_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::CharacterSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_character for token={}", body.vote_token);
    let client = reqwest::Client::new();
    let sanitized = client.post(&format!("http://{}/v1/character/", SUBMIT_VALIDATOR)).json(&body.0).send().await?.json::<models::CharacterSubmitRest>().await?;
    service.submitCharcater(sanitized).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_music_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::MusicSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_music for token={}", body.vote_token);
    let client = reqwest::Client::new();
    let sanitized = client.post(&format!("http://{}/v1/music/", SUBMIT_VALIDATOR)).json(&body.0).send().await?.json::<models::MusicSubmitRest>().await?;
    service.submitMusic(sanitized).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_cp_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::CPSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_cp for token={}", body.vote_token);
    let client = reqwest::Client::new();
    let sanitized = client.post(&format!("http://{}/v1/cp/", SUBMIT_VALIDATOR)).json(&body.0).send().await?.json::<models::CPSubmitRest>().await?;
    service.submitCP(sanitized).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_work_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::WorkSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_work for token={}", body.vote_token);
    let client = reqwest::Client::new();
    let sanitized = client.post(&format!("http://{}/v1/work/", SUBMIT_VALIDATOR)).json(&body.0).send().await?.json::<models::WorkSubmitRest>().await?;
    service.submitWork(sanitized).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

pub async fn submit_paper_v1(service: SubmitServiceV1Wrapper, body: actix_web::web::Json<models::PaperSubmitRest>) -> Result<HttpResponse, Error> {
    println!("submit_paper for token={}", body.vote_token);
    let client = reqwest::Client::new();
    let sanitized = client.post(&format!("http://{}/v1/paper/", SUBMIT_VALIDATOR)).json(&body.0).send().await?.json::<models::PaperSubmitRest>().await?;
    service.submitPaper(sanitized).await?;
    Ok(HttpResponse::Ok().json(PostResult::new()))
}

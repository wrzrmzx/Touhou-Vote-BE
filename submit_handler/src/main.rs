
use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};

mod shared;
mod models;
mod handlers;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "actix_web=debug");
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("submits");

    // Start http server
    HttpServer::new(move || {
        let submit_service = services::SubmitService::new(db.clone());
        App::new()
            .data(submit_service)
            .route("/character/", web::post().to(handlers::submit_character))
            .route("/music/", web::post().to(handlers::submit_music))
            .route("/cp/", web::post().to(handlers::submit_cp))
            .route("/work/", web::post().to(handlers::submit_work))
            .route("/paper/", web::post().to(handlers::submit_paper))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}

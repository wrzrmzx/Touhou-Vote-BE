
use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
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

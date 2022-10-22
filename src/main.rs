mod app;

use actix_web::{ App, HttpServer };
use app::user::{create_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_user))
        .bind(("127.0.0.1", 7000))?
        .run()
        .await
}

use actix_files::Files;
use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_directory = "./static";

    let port = 8000;

    println!("Serving files at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new().service(Files::new("/", file_directory).show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

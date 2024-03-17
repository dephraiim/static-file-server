use actix_files::Files;
use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <directory>");
        return Ok(());
    }

    let file_directory = args[1].clone();
    let port = 3000;

    println!(
        "Serving files from {} at http://localhost:{}",
        file_directory, port
    );

    HttpServer::new(move || {
        App::new().service(Files::new("/", &file_directory).show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

use actix_files::Files;
use actix_web::{App, HttpServer};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    directory: String,

    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file_directory = args.directory;
    let port = args.port;

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

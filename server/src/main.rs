use actix_web::{get, App, HttpServer, Responder};
use actix_files::{Files, NamedFile};

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open("static/index.html")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving static files on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

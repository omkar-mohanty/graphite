use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use lib::Database;

struct Application {
    db: Database,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .service(hello)
    }).bind(("127.0.0.1", 8080))?.run().await?;
    Ok(())
}

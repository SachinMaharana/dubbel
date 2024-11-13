use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
pub fn run() -> Result<Server, std::io::Error>{
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))

    })
        .bind("0.0.0.0:8000")?
        .run();

    Ok(server)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
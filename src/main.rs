use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))

    }).bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
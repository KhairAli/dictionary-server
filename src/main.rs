
#[path = "service/image_service.rs"] mod image_service;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/image/text")]
pub async fn get_text() -> impl Responder {
    HttpResponse::Ok().body(image_service::image_to_text())
}







#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_text)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



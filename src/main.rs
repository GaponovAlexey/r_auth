use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(service::start)
            .service(service::get_user)
            .service(service::create_user)
            .service(service::basic_auth)
            .service(service::create_article)
    })
        .bind(("127.0.0.1", 3000))?
        .run().await
}

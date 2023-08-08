use actix_web::{ get, post, web::{ Data }, Responder, HttpResponse };

pub struct AppState {}

#[get("/user")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/")]
async fn start() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/user")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/auth")]
async fn basic_auth() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/article")]
async fn create_article() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

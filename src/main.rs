use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User{
    id: u8,
    name: String,
    email: String,
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("hello world")
}

#[post("/user")]
async fn buat_user(user: web::Json<User>) -> impl Responder{
    HttpResponse::Created().json(user.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(buat_user)
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}

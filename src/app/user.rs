use actix_web::{post, HttpResponse, web};

#[post("/user")]
pub async fn create_user(data: web::Data<Container>) -> HttpResponse {
    HttpResponse::Created().body("User created with successul")
}

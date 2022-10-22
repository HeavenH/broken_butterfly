use actix_web::{post, HttpResponse};

use crate::core::common::error::ErrorReponse;
use crate::core::user::usecase::{create_new_user};

#[post("/user")]
pub async fn create_user() ->  Result<HttpResponse, ErrorReponse> {
    let create_user_use_case =
}

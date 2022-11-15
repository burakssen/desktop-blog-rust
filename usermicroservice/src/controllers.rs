use std::fmt::format;

use super::service::models::{NewUser, User};
use super::service::{create_user as service_create_user, get_all_users as service_get_all_users};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/get_all_users")]
pub async fn get_all_users() -> impl Responder {
    eprintln!("Get all users called");
    service_get_all_users()
}

#[post("/create_user")]
pub async fn create_user(data: web::Json<NewUser>) -> impl Responder {
    let response: User = service_create_user(
        data.user_name.to_string(),
        data.password.to_string(),
        data.mail.to_string(),
    );
    format!("{}", response.id)
}

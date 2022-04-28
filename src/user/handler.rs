use crate::db::PgPool;
use crate::lib::error::{ErrResponse, ErrType};
use crate::user::{model, request};

use actix_web::{get, post, put, web, HttpResponse};

#[get("/")]
async fn get_all_host(pool: web::Data<PgPool>) -> HttpResponse {
  let host_list = model::User::get_all(pool);

  match host_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
  }
}

#[post("/")]
async fn insert_new_host(
  body: web::Json<request::UserRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  match model::User::add(body, pool) {
    Ok(res) => HttpResponse::Ok().body(format!("Affected Rows: {}", res)),
    Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
  }
}

#[put("/")]
async fn update_host(
  body: web::Json<request::UserRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  match model::User::update(body, pool) {
    Ok(res) => HttpResponse::Ok().json(res),
    Err(e) => ErrResponse::new(ErrType::BadRequest, e.to_string()),
  }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config
    .service(get_all_host)
    .service(insert_new_host)
    .service(update_host);
}

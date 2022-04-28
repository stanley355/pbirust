use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum ErrType {
  BadRequest,
  InternalServerError,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrResponse {
  code: u16,
  error: String,
  message: String,
}

impl ErrResponse {
  pub fn new(error_type: ErrType, error: String) -> HttpResponse {
    let error_res = ErrResponse {
      code: ErrResponse::status_code(error_type.clone()).as_u16(),
      error: error,
      message: ErrResponse::error_message(error_type.clone()),
    };

    error_res.return_httpresponse(error_type)
  }

  fn status_code(error_type: ErrType) -> StatusCode {
    match error_type {
      ErrType::BadRequest => StatusCode::BAD_REQUEST,
      ErrType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_message(error_type: ErrType) -> String {
    match error_type {
      ErrType::BadRequest => "Bad Request".to_string(),
      ErrType::InternalServerError => "Internal Server Error".to_string(),
    }
  }

  fn return_httpresponse(self, error_type: ErrType) -> HttpResponse {
    match error_type {
      ErrType::BadRequest => HttpResponse::BadRequest().json(self),
      ErrType::InternalServerError => HttpResponse::InternalServerError().json(self),
    }
  }
}

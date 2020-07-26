use actix_web::{error, error::ResponseError,
                Result, HttpRequest, HttpResponse, FromRequest};
use actix_web::web::{Json, ServiceConfig, post, resource};

use actix_web::dev::{HttpResponseBuilder};
use actix_web::http::StatusCode;
use serde::ser::SerializeStruct;

use std::fmt;

mod business_logic;

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(resource("/api/v1/business_logic")
              .app_data(Json::<business_logic::LogicRequest>::configure(|cfg| {
                cfg.limit(4096).error_handler(json_error_handler)
              }))
              .route(post().to(business_logic::execute))
  );
}

fn json_error_handler(
  err: error::JsonPayloadError,
  _req: &HttpRequest
) -> error::Error {
  let detail = err.to_string();
  let body =
    serde_json::to_string(&Error{code: StatusCode::BAD_REQUEST,
                                 error: detail})
    .unwrap();
  println!("{:?}", body);

  let response =
    HttpResponse::BadRequest()
    .content_type("application/json")
    .body(body);

  error::InternalError::from_response(err, response).into()
}

#[derive(Debug)]
pub struct Error {
  code: StatusCode,
  error: String
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.code, self.error)
  }
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    HttpResponseBuilder::new(self.code)
      .body(serde_json::to_string(self).unwrap())
  }
  fn status_code(&self) -> StatusCode { self.code }
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut s = serializer.serialize_struct("Error", 2)?;
    s.serialize_field("code", &self.code.as_u16())?;
    s.serialize_field("error", &self.error)?;
    s.end()
  }
}

mod traits {
  pub trait Error {
    fn to_error(&self) -> super::Error;
  }
}

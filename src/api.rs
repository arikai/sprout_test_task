use actix_web::{Result, HttpResponse};
use actix_web::error::ResponseError;
use actix_web::dev::{HttpResponseBuilder};
use actix_web::web::Json;
use serde_derive::Deserialize;
use actix_web::http::StatusCode;
use serde::ser::SerializeStruct;

use std::fmt;

#[path = "./business_logic.rs"]
mod business_logic;

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

mod traits {
  pub trait Error {
    fn to_error(&self) -> super::Error;
  }
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

#[derive(Debug, Deserialize)]
pub struct LogicRequest {
  a: bool,
  b: bool,
  c: bool,
  d: f64,
  e: i64,
  f: i64
}

impl traits::Error for business_logic::ExecutionError {
  fn to_error(&self) -> Error {
    let msg =
      match *self {
        business_logic::ExecutionError::NoRuleError => "No rule found"
      };
    Error {
      code: StatusCode::BAD_REQUEST,
      error: msg.to_string()
    }
  }
}

use traits::Error as ErrorTrait;

pub async fn business_logic(r: Json<LogicRequest>)
                            -> Result<Json<business_logic::LogicResult>, Error>
{
  let r = r.into_inner();
  business_logic::execute(r.a, r.b, r.c, r.d, r.e, r.f)
    .map(|res| Json(res))
    .map_err(|err| err.to_error())
}

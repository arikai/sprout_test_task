use serde_derive::Deserialize;
use actix_web::http::StatusCode;
use actix_web::web::Json;

use super::*;
use traits::Error as ErrorTrait;
#[path = "../business_logic.rs"]
mod business_logic;


#[derive(Debug, Deserialize)]
pub struct LogicRequest {
  a: bool,
  b: bool,
  c: bool,
  d: f64,
  e: i64,
  f: i64
}

impl ErrorTrait for business_logic::ExecutionError {
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


pub async fn execute(r: Json<LogicRequest>)
                            -> Result<Json<business_logic::LogicResult>, Error>
{
  let r = r.into_inner();
  business_logic::execute(r.a, r.b, r.c, r.d, r.e, r.f)
    .map(|res| Json(res))
    .map_err(|err| err.to_error())
}

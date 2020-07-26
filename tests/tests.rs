extern crate sprout_test_task;

#[cfg(test)]
mod tests {
  use sprout_test_task::api;
  use actix_web::{App, test, test::TestRequest,
                  http::header,
                  http::StatusCode,
                  dev::ServiceResponse};
  use serde_json::{json, Value as JsonValue};

  async fn request(json: JsonValue) -> ServiceResponse {
    let payload = json.to_string();
    let mut app = test::init_service(App::new().configure(api::config)).await;

    let req =
      TestRequest::post()
      .uri("/api/v1/business_logic")
      .header(header::CONTENT_TYPE, "application/json")
      .set_payload(payload)
      .to_request();

    test::call_service(&mut app, req).await
  }

  #[actix_rt::test]
  async fn test_simple() {
    let json = json!({
      "a": true,
      "b": true,
      "c": false,
      "d": 1.0,
      "e": 2,
      "f": 3
    });

    let resp = request(json).await;

    assert!(resp.status().is_success());
    assert_eq!(test::read_body(resp).await,
               r#"{"h":1024,"k":0.9}"#.as_bytes())
  }

  #[actix_rt::test]
  async fn test_no_rule() {
    let json = json!({
      "a": false,
      "b": false,
      "c": false,
      "d": 1.0,
      "e": 2,
      "f": 3
    });

    let resp = request(json).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(test::read_body(resp).await,
               r#"{"code":400,"error":"No rule found"}"#.as_bytes())
  }

  #[actix_rt::test]
  async fn test_bad_params() {
    let json = json!({
      "a": false,
      "b": false,
      "c": 1.0,
      "d": 1.0,
      "e": 2,
      "f": 3
    });

    let resp = request(json).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(test::read_body(resp).await,
               r#"{"code":400,"error":"Json deserialize error: invalid type: floating point `1`, expected a boolean at line 1 column 28"}"#.as_bytes())
  }
}

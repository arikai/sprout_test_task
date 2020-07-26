use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use std::env;

mod api;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
  setup_logger();

  let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
  let port = env::var("PORT").unwrap_or("4000".to_string());

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .configure(api::config)
  })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

fn setup_logger() {
  env::set_var("RUST_LOG", "actix_web=debug");
  env_logger::init();
}

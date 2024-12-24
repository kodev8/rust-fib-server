use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use std::sync::Mutex;

mod server_math;
use server_math::store::RedisStore;
use server_math::req_resp::{BasicResponse, AppState};
use server_math::factorial::calc_factorial;
use server_math::fibonacci::calc_fib;

// In main.rs:
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());


    HttpServer::new(move || {
        let cors = Cors::permissive();

        // Initialize Redis stores
        let fib_store = Box::new(RedisStore::new(&redis_url, "fib")
            .expect("Failed to create fibonacci Redis store"));
        let fact_store = Box::new(RedisStore::new(&redis_url, "fact")
            .expect("Failed to create factorial Redis store"));

        let app_state = web::Data::new(AppState {
            fib_store: web::Data::new(Mutex::new(fib_store)),
            fact_store: web::Data::new(Mutex::new(fact_store)),
        });

        App::new()
            .wrap(cors)
            .app_data(app_state)
            .service(health)
            .service(calc_fib)
            .service(calc_factorial)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(BasicResponse {
        message: "Service is up and running".to_string(),
    })
}




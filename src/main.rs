use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use env_logger::Env;
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Serialize)]
struct Fib {
    message: String,
    #[serde(with = "bigint_serialize")]
    fib: BigInt,
    cached: bool,
}

#[derive(Deserialize)]
struct FibRequest {
    num: Option<i64>,
}

#[derive(Serialize)]
struct HealthResponse {
    message: String,
}

// Serialization helper for BigInt
mod bigint_serialize {
    use num_bigint::BigInt;
    use serde::{Serializer, Serialize};

    pub fn serialize<S>(num: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        num.to_string().serialize(serializer)
    }
}

#[derive(Clone)]
struct AppState {
    fib_store: web::Data<Mutex<HashMap<i64, BigInt>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::permissive();

        let mut store = HashMap::new();
        store.insert(0, BigInt::zero());
        store.insert(1, BigInt::one());

        let app_state = web::Data::new(AppState {
            fib_store: web::Data::new(Mutex::new(store)),
        });

        App::new()
            .wrap(cors)
            .app_data(app_state)
            .service(health)
            .service(calc_fib)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 9100))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(HealthResponse {
        message: "Service is up and running".to_string(),
    })
}

#[get("/fib")]
async fn calc_fib(
    fib_request: web::Query<FibRequest>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let num = fib_request.num;
    let mut store = data.fib_store.lock().unwrap();

    match num {
        Some(num) => {
            if num < 0 {
                return Ok(web::Json(Fib {
                    message: "Number must be non-negative".to_string(),
                    fib: BigInt::zero(),
                    cached: false,
                }));
            }

            let (result, was_cached) = match store.get(&num) {
                Some(fib) => (fib.clone(), true),
                None => (find_nth_fibonacci(num, &mut store), false),
            };

            let message = if was_cached {
                format!("Fibonacci number {} retrieved from cache", num)
            } else {
                format!("Fibonacci number {} calculated", num)
            };

            Ok(web::Json(Fib {
                message,
                fib: result,
                cached: was_cached,
            }))
        }
        None => Ok(web::Json(Fib {
            message: "No number provided".to_string(),
            fib: BigInt::from(-1),
            cached: false,
        })),
    }
}

fn find_nth_fibonacci(num: i64, store: &mut HashMap<i64, BigInt>) -> BigInt {
    if num <= 1 {
        return if num == 0 { BigInt::zero() } else { BigInt::one() };
    }

    // Find the largest calculated Fibonacci number in our store
    let mut max_calculated = 1;
    for i in 0..=num {
        if store.contains_key(&i) {
            max_calculated = i;
        } else {
            break;
        }
    }

    // Calculate remaining numbers iteratively
    let mut current = store.get(&max_calculated).unwrap().clone();
    let mut prev = store.get(&(max_calculated - 1)).unwrap().clone();

    for n in (max_calculated + 1)..=num {
        let next = current.clone() + prev.clone();
        store.insert(n, next.clone());
        prev = current;
        current = next;
    }

    store.get(&num).unwrap().clone()
}
use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use env_logger::Env;
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Serialize, Deserialize)]
struct Fib {
    message: String,
    fib: BigInt,
    cached: bool,
}

#[derive(Serialize, Deserialize)]
struct FibRequest {
    num: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    message: String,
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


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{dev::{Service, ServiceResponse}, test, App};
    use actix_http::Request;

   
    async fn create_test_app() -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
        let mut store = HashMap::new();
        store.insert(0, BigInt::zero());
        store.insert(1, BigInt::one());

        let app_state = web::Data::new(AppState {
            fib_store: web::Data::new(Mutex::new(store)),
        });

        test::init_service(
            App::new()
                .app_data(app_state)
                .service(health)
                .service(calc_fib)
        ).await
    }

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = create_test_app().await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp: HealthResponse = test::call_and_read_body_json(&app, req).await;
        
        assert_eq!(resp.message, "Service is up and running");
    }

    #[actix_web::test]
    async fn test_fibonacci_basic_numbers() {
        let app = create_test_app().await;

        // Test first few Fibonacci numbers
        let test_cases = vec![
            (0, "0"),
            (1, "1"),
            (2, "1"),
            (3, "2"),
            (4, "3"),
            (5, "5"),
            (6, "8"),
            (7, "13"),
        ];

        for (input, expected) in test_cases {
            let req = test::TestRequest::get()
                .uri(&format!("/fib?num={}", input))
                .to_request();
            let resp: Fib = test::call_and_read_body_json(&app, req).await;
            
            assert_eq!(resp.fib.to_string(), expected);
            
            // First call should not be cached
            if input <= 1 {
                assert!(resp.cached, "Base cases (0,1) should be cached initially");
            } else {
                assert!(!resp.cached, "First calculation should not be cached");
            }

            // Second call should be cached
            let req2 = test::TestRequest::get()
                .uri(&format!("/fib?num={}", input))
                .to_request();
            let resp2: Fib = test::call_and_read_body_json(&app, req2).await;
            assert!(resp2.cached, "Second call should be cached");
        }
    }

    #[actix_web::test]
    async fn test_fibonacci_negative_number() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/fib?num=-1")
            .to_request();
        let resp: Fib = test::call_and_read_body_json(&app, req).await;
        
        assert_eq!(resp.fib.to_string(), "0");
        assert_eq!(resp.message, "Number must be non-negative");
        assert!(!resp.cached);
    }

    #[actix_web::test]
    async fn test_fibonacci_no_number() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/fib")
            .to_request();
        let resp: Fib = test::call_and_read_body_json(&app, req).await;
        
        assert_eq!(resp.fib.to_string(), "-1");
        assert_eq!(resp.message, "No number provided");
        assert!(!resp.cached);
    }

    #[actix_web::test]
    async fn test_fibonacci_larger_number() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/fib?num=20")
            .to_request();
        let resp: Fib = test::call_and_read_body_json(&app, req).await;
        
        assert_eq!(resp.fib.to_string(), "6765"); // 20th Fibonacci number
        assert!(!resp.cached);

        // Verify intermediate results are cached
        for i in 2..=20 {
            let req = test::TestRequest::get()
                .uri(&format!("/fib?num={}", i))
                .to_request();
            let resp: Fib = test::call_and_read_body_json(&app, req).await;
            assert!(resp.cached, "Number {} should be cached", i);
        }
    }

    #[actix_web::test]
    async fn test_fibonacci_caching_behavior() {
        let app = create_test_app().await;

        // First request - should calculate and cache
        let req1 = test::TestRequest::get()
            .uri("/fib?num=10")
            .to_request();
        let resp1: Fib = test::call_and_read_body_json(&app, req1).await;
        assert!(!resp1.cached);
        assert_eq!(resp1.fib.to_string(), "55");

        // Second request - should use cache
        let req2 = test::TestRequest::get()
            .uri("/fib?num=10")
            .to_request();
        let resp2: Fib = test::call_and_read_body_json(&app, req2).await;
        assert!(resp2.cached);
        assert_eq!(resp2.fib.to_string(), "55");
        assert!(resp2.message.contains("retrieved from cache"));

        // Check that intermediate values are cached
        let req3 = test::TestRequest::get()
            .uri("/fib?num=5")
            .to_request();
        let resp3: Fib = test::call_and_read_body_json(&app, req3).await;
        assert!(resp3.cached);
        assert_eq!(resp3.fib.to_string(), "5");
    }
}
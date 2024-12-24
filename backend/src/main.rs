use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::sync::Mutex;

mod server_math;
use server_math::{
    factorial::calc_factorial,
    fibonacci::calc_fib,
    req_resp::{BasicResponse, AppState}
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::permissive();

        let mut fib_store = HashMap::new();
        fib_store.insert(0, BigInt::zero());
        fib_store.insert(1, BigInt::one());

        let mut fact_store = HashMap::new();
        fact_store.insert(0, BigInt::one());
        fact_store.insert(1, BigInt::one());

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
    .bind(("0.0.0.0", 8003))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> impl Responder {
    web::Json(BasicResponse {
        message: "Service is up and running".to_string(),
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::Request;
    use actix_web::{
        dev::{Service, ServiceResponse},
        test, App,
    };
    use server_math::req_resp::MathResponse;

    async fn create_test_app(
    ) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
        let mut fib_store = HashMap::new();
        fib_store.insert(0, BigInt::zero());
        fib_store.insert(1, BigInt::one());

        let mut fact_store = HashMap::new();
        fact_store.insert(0, BigInt::one());
        fact_store.insert(1, BigInt::one());

        let app_state = web::Data::new(AppState {
            fib_store: web::Data::new(Mutex::new(fib_store)),
            fact_store: web::Data::new(Mutex::new(fact_store)),
        });

        test::init_service(
            App::new()
                .app_data(app_state)
                .service(health)
                .service(calc_fib)
                .service(calc_factorial),
                
        )
        .await
    }

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = create_test_app().await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp: BasicResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.message, "Service is up and running");
    }

    // integration test for fibonacci sequence
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

            let resp: MathResponse = test::call_and_read_body_json(&app, req).await;

            assert_eq!(resp.result, expected.to_string());

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

            let resp2: MathResponse = test::call_and_read_body_json(&app, req2).await;

            assert!(resp2.cached, "Second call should be cached");
        }
    }

    #[actix_web::test]
    async fn test_fibonacci_negative_number() {
        let app = create_test_app().await;
        let req = test::TestRequest::get().uri("/fib?num=-1").to_request();
        let resp: BasicResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.message, "Number must be non-negative");
    }

    #[actix_web::test]
    async fn test_fibonacci_larger_number() {
        let app = create_test_app().await;
        let req = test::TestRequest::get().uri("/fib?num=20").to_request();
        let resp: MathResponse = test::call_and_read_body_json(&app, req).await;

        // 20th fib should be 6765
        assert_eq!(resp.result, "6765".to_string());
        assert!(!resp.cached);

        // Verify intermediate results are cached
        for i in 2..=20 {
            let req = test::TestRequest::get()
                .uri(&format!("/fib?num={}", i))
                .to_request();
            let resp: MathResponse = test::call_and_read_body_json(&app, req).await;
            assert!(resp.cached, "Number {} should be cached", i);
        }
    }

    #[actix_web::test]
    async fn test_fibonacci_caching_behavior() {
        let app = create_test_app().await;

        // First request should calculate the result and cache it for the second request
        let req1 = test::TestRequest::get().uri("/fib?num=10").to_request();
        let resp1: MathResponse = test::call_and_read_body_json(&app, req1).await;

        assert!(!resp1.cached);
        assert_eq!(resp1.result, "55".to_string());

        let req2 = test::TestRequest::get().uri("/fib?num=10").to_request();

        let resp2: MathResponse = test::call_and_read_body_json(&app, req2).await;

        assert!(resp2.cached);
        assert_eq!(resp2.result, "55".to_string());
        assert!(resp2.message.contains("retrieved from cache"));

        let req3 = test::TestRequest::get().uri("/fib?num=5").to_request();
        let resp3: MathResponse = test::call_and_read_body_json(&app, req3).await;
        assert!(resp3.cached);
        assert_eq!(resp3.result,"5".to_string());
    }


    // integration test for factorial
    #[actix_web::test]
    async fn test_factorial_basic_numbers() {
        let app = create_test_app().await;

        // Test first few factorials
        let test_cases = vec![
            // (0, "1"),
            (1, "1"),
            (2, "2"),
            (3, "6"),
            (4, "24"),
            (5, "120"),
            (6, "720"),
            (7, "5040"),
        ];

        for (input, expected) in test_cases {
            let req = test::TestRequest::get()
                .uri(&format!("/factorial?num={}", input))
                .to_request();

            let resp: MathResponse = test::call_and_read_body_json(&app, req).await;

            assert_eq!(resp.result, expected.to_string());

            // First call should not be cached
            if input <= 1 {
                assert!(resp.cached, "Base cases (0,1) should be cached initially");
            } else {
                assert!(!resp.cached, "First calculation should not be cached");
            }

            // Second call should be cached
            let req2 = test::TestRequest::get()
                .uri(&format!("/factorial?num={}", input))
                .to_request();

            let resp2: MathResponse = test::call_and_read_body_json(&app, req2).await;

            assert!(resp2.cached, "Second call should be cached");
        }
    }

    #[actix_web::test]
    async fn test_factorial_negative_number() {
        let app = create_test_app().await;
        let req = test::TestRequest::get().uri("/factorial?num=-1").to_request();
        let resp: BasicResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.message, "Number must be non-negative");
    }
}






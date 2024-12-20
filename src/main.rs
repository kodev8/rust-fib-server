use actix_web::{get, web, App, HttpServer, Responder, Result};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize)]
struct Fib {
    message: String,
    fib: usize,
}

#[derive(Deserialize)]
struct FibRequest {
    num: i32,
}

#[derive(Serialize)]
struct HealthResponse {
    message: String,
}

// Make Fibonacci Clone-able so it can be shared between threads
#[derive(Clone)]
struct AppState {
    fib_store: web::Data<Mutex<HashMap<i32, i32>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the fibonacci store with base cases
    let mut store = HashMap::new();
    store.insert(0, 0);
    store.insert(1, 1);
    
    // Wrap the store in web::Data and Mutex for thread-safe sharing
    let app_state = web::Data::new(AppState {
        fib_store: web::Data::new(Mutex::new(store)),
    });

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(health)
            .service(calc_fib)
    })
    .bind(("127.0.0.1", 8080))?
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

    // Check if we already have the result cached
    if let Some(&result) = store.get(&num) {
        return Ok(web::Json(Fib {
            message: "Fibonacci (cached)".to_string(),
            fib: result as usize,
        }));
    }

    // Calculate new Fibonacci number
    let result = find_nth_fibonacci(num, &mut store);
    
    Ok(web::Json(Fib {
        message: "Fibonacci".to_string(),
        fib: result as usize,
    }))
}

fn find_nth_fibonacci(num: i32, store: &mut HashMap<i32, i32>) -> i32 {
    if num <= 1 {
        return num;
    }

    let mut fib_sum = 0;
    let mut previous = 1;

    for i in 0..num {
        let temp = previous;
        previous = fib_sum;
        fib_sum = fib_sum + temp;
        store.insert(i + 1, fib_sum);
    }

    fib_sum
}
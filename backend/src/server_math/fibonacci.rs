use crate::server_math::req_resp::{MathResponse, NumRequest, AppState, BasicResponse};
use actix_web::{get, web, Responder, Result, HttpResponse};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::collections::HashMap;

fn find_nth_fibonacci(num: i64, store: &mut HashMap<i64, BigInt>) -> Result<(BigInt, bool), String> {

    if num < 0 {
        return Err("Number must be non-negative".to_string());
    }

    if num <= 1 {
        return if num == 0 {
            Ok((BigInt::zero(), true))
        } else {
            Ok((BigInt::one(), true))
        };
    }

    if store.contains_key(&num) {
        return Ok((store.get(&num).unwrap().clone(), true));
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

    Ok((store.get(&num).unwrap().clone(), false))
}


#[get("/fib")]
async fn calc_fib(
    fib_request: web::Query<NumRequest>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let num = fib_request.num;
    let mut store = data.fib_store.lock().unwrap();

    match num {
        Some(num) => {
            match find_nth_fibonacci(num, &mut store) {
                Ok((result, was_cached)) => {
                    let message = if was_cached {
                        format!("Fibonacci number {} retrieved from cache", num)
                    } else {
                        format!("Fibonacci number {} calculated", num)
                    };

                    Ok(HttpResponse::Ok().json(MathResponse {
                        message,
                        result: result.to_string(),
                        cached: was_cached,
                    }))
                }
                Err(e) => Ok(HttpResponse::BadRequest().json(BasicResponse { message: e })),
            }
        }
        None => Err(actix_web::error::ErrorBadRequest("Missing 'num' parameter")),
    }
}
// unit test find nth fibonacci
#[cfg(test)]
mod fib_tests { 
    use super::*;

    fn setup_test_store() -> HashMap<i64, BigInt> {
        let mut store = HashMap::new();
        store.insert(0, BigInt::zero());
        store.insert(1, BigInt::one());
        store
    }

    #[test]
    fn test_find_nth_fibonacci() {
        let mut store = setup_test_store();
        let mut num = 10;
        let result = find_nth_fibonacci(num, &mut store);
        assert_eq!(result, Ok((BigInt::from(55), false)));
        num = 20;
        let result = find_nth_fibonacci(num, &mut store);
        assert_eq!(result, Ok((BigInt::from(6765), false)));
    }

    #[test]
    fn test_negative_number() {
        let mut store = setup_test_store();
        let num = -1;
        let result = find_nth_fibonacci(num, &mut store);
        assert_eq!(result, Err("Number must be non-negative".to_string()));
    }
}

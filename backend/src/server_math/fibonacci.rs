use crate::server_math::req_resp::{MathResponse, NumRequest, AppState, BasicResponse};
use crate::server_math::store::Store;
use actix_web::{get, web, Responder, Result, HttpResponse};
use num_bigint::BigInt;
use num_traits::{Zero, One};

fn find_nth_fibonacci<S: Store>(num: i64, store: &mut S) -> Result<(BigInt, bool), String> {
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

    // Check if result is in cache
    if store.contains_key(num)? {
        if let Some(result) = store.get(num)? {
            return Ok((result, true));
        }
    }

    // Find the largest calculated Fibonacci number in our store
    let mut max_calculated = 1;
    for i in 0..=num {
        if store.contains_key(i)? {
            max_calculated = i;
        } else {
            break;
        }
    }

    // Get the last two calculated numbers

    let mut current = store.get(max_calculated)?.unwrap_or_else(BigInt::one);
    let mut prev = store.get(max_calculated - 1)?.unwrap_or_else(BigInt::zero);

    // Calculate remaining numbers iteratively
    for n in (max_calculated + 1)..=num {
        let next = current.clone() + prev.clone();
        store.set(n, &next)?;
        prev = current;
        current = next;
    }

    store.get(num)?
        .map(|result| (result, false))
        .ok_or_else(|| "Failed to retrieve calculated result".to_string())
}

#[get("/fib")]
async fn calc_fib(
    fib_request: web::Query<NumRequest>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let num = fib_request.num.unwrap_or(0);
    let mut store = data.fib_store.lock().unwrap();

    match find_nth_fibonacci(num, &mut *store) {
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

#[cfg(test)]
mod fib_tests {
    use super::*;
    use crate::server_math::store::HashMapStore;
    use num_traits::{Zero, One};

    fn setup_test_store() -> HashMapStore {
        let mut store = HashMapStore::new();
        
        // Initialize base cases
        let _ = store.set(0, &BigInt::zero());
        let _ = store.set(1, &BigInt::one());
        
        store
    }

    #[test]
    fn test_find_fibonacci_basic() {
        let mut store = setup_test_store();
        
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
            let (result, _) = find_nth_fibonacci(input, &mut store).unwrap();
            assert_eq!(result.to_string(), expected);
        }
    }

    #[test]
    fn test_find_fibonacci_caching() {
        let mut store = setup_test_store();

        // First calculation - should not be cached
        let (result1, was_cached1) = find_nth_fibonacci(5, &mut store).unwrap();
        assert_eq!(result1.to_string(), "5");
        assert!(!was_cached1);

        // Second calculation - should be cached
        let (result2, was_cached2) = find_nth_fibonacci(5, &mut store).unwrap();
        assert_eq!(result2.to_string(), "5");
        assert!(was_cached2);

        // Check intermediate results are cached
        let (result3, was_cached3) = find_nth_fibonacci(3, &mut store).unwrap();
        assert_eq!(result3.to_string(), "2");
        assert!(was_cached3);
    }

    #[test]
    fn test_negative_number() {
        let mut store = setup_test_store();
        match find_nth_fibonacci(-1, &mut store) {
            Ok(_) => panic!("Expected error for negative number"),
            Err(e) => assert_eq!(e, "Number must be non-negative"),
        }
    }

    #[test]
    fn test_fibonacci_larger_number() {
        let mut store = setup_test_store();
        let (result, _) = find_nth_fibonacci(10, &mut store).unwrap();
        assert_eq!(result.to_string(), "55");
    }

    #[test]
    fn test_sequential_calculations() {
        let mut store = setup_test_store();
        
        // Calculate fib of 5
        let (result5, cached5) = find_nth_fibonacci(5, &mut store).unwrap();
        assert_eq!(result5.to_string(), "5");
        assert!(!cached5);

        // Calculate fib of 7 - should use cached results up to 5
        let (result7, cached7) = find_nth_fibonacci(7, &mut store).unwrap();
        assert_eq!(result7.to_string(), "13");
        assert!(!cached7);

        // Verify all intermediate results are cached
        for i in 0..=7 {
            let (_, cached) = find_nth_fibonacci(i, &mut store).unwrap();
            assert!(cached, "Fibonacci number {} should be cached", i);
        }
    }
}
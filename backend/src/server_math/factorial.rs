use crate::server_math::req_resp::{MathResponse, NumRequest, AppState, BasicResponse};
use crate::server_math::store::Store;
use actix_web::{get, web, Responder, Result, HttpResponse};
use num_bigint::BigInt;
use num_traits::One;

fn find_factorial<S: Store>(num: i64, store: &mut S) -> Result<(BigInt, bool), String> {

    if num < 0 {
        return Err("Number must be non-negative".to_string());
    }

    // Check if result is in cache
    if store.contains_key(num)? {
        if let Some(result) = store.get(num)? {
            return Ok((result, true));
        }
    }

    // Find the largest calculated factorial in our store
    let mut max_calculated = 0;
    for i in 0..=num {
        if store.contains_key(i)? {
            max_calculated = i;
        } else {
            break;
        }
    }

    // Get the last calculated factorial
    let mut result = store.get(max_calculated)?
        .unwrap_or_else(BigInt::one);

    // Calculate remaining numbers iteratively
    for i in (max_calculated + 1)..=num {
        result *= i;
        store.set(i, &result)?;
    }

    Ok((result, false))
}

#[get("/factorial")]
async fn calc_factorial(
    fact_request: web::Query<NumRequest>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let num = fact_request.num.unwrap_or(0);
    let mut store = data.fact_store.lock().unwrap();

    match find_factorial(num, &mut *store) {
        Ok((result, was_cached)) => {
            let message = if was_cached {
                format!("Factorial of {} retrieved from cache", num)
            } else {
                format!("Factorial of {} calculated", num)
            };
            
            Ok(HttpResponse::Ok().json(MathResponse {
                message,
                result: result.to_string(),
                cached: was_cached,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(BasicResponse {
                message: e,
            }))
        }
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;
    use crate::server_math::store::HashMapStore;
    use num_traits::One;

    fn setup_test_store() -> HashMapStore {
        let mut store = HashMapStore::new();
        
        // Initialize base cases
        let _ = store.set(0, &BigInt::one());
        let _ = store.set(1, &BigInt::one());
        
        store
    }

    #[test]
    fn test_find_factorial_basic() {
        let mut store = setup_test_store();
        
        let test_cases = vec![
            (0, "1"),
            (1, "1"),
            (2, "2"),
            (3, "6"),
            (4, "24"),
            (5, "120"),
        ];

        for (input, expected) in test_cases {
            let (result, _) = find_factorial(input, &mut store).unwrap();
            assert_eq!(result.to_string(), expected);
        }
    }

    #[test]
    fn test_find_factorial_caching() {
        let mut store = setup_test_store();

        // First calculation - should not be cached
        let (result1, was_cached1) = find_factorial(5, &mut store).unwrap();
        assert_eq!(result1.to_string(), "120");
        assert!(!was_cached1);

        // Second calculation - should be cached
        let (result2, was_cached2) = find_factorial(5, &mut store).unwrap();
        assert_eq!(result2.to_string(), "120");
        assert!(was_cached2);

        // Check intermediate results are cached
        let (result3, was_cached3) = find_factorial(3, &mut store).unwrap();
        assert_eq!(result3.to_string(), "6");
        assert!(was_cached3);
    }

    #[test]
    fn test_negative_number() {
        let mut store = setup_test_store();
        match find_factorial(-1, &mut store) {
            Ok(_) => panic!("Expected error for negative number"),
            Err(e) => assert_eq!(e, "Number must be non-negative"),
        }
    }

    #[test]
    fn test_factorial_larger_number() {
        let mut store = setup_test_store();
        let (result, _) = find_factorial(10, &mut store).unwrap();
        assert_eq!(result.to_string(), "3628800");
    }

    #[test]
    fn test_sequential_calculations() {
        let mut store = setup_test_store();
        
        // Calculate factorial of 5
        let (result5, cached5) = find_factorial(5, &mut store).unwrap();
        assert_eq!(result5.to_string(), "120");
        assert!(!cached5);

        // Calculate factorial of 7 - should use cached results up to 5
        let (result7, cached7) = find_factorial(7, &mut store).unwrap();
        assert_eq!(result7.to_string(), "5040");
        assert!(!cached7);

        // Verify all intermediate results are cached
        for i in 0..=7 {
            let (_, cached) = find_factorial(i, &mut store).unwrap();
            assert!(cached, "Factorial of {} should be cached", i);
        }
    }
}
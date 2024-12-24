use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use num_bigint::BigInt;
use actix_web::web;


#[derive(Serialize, Deserialize)]
pub struct MathResponse {
    pub message: String,
    pub result: String,
    pub cached: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NumRequest {
    pub num: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct BasicResponse {
    pub message: String,
}

#[derive(Clone)]
pub struct AppState {
    pub fib_store: web::Data<Mutex<HashMap<i64, BigInt>>>,
    pub fact_store: web::Data<Mutex<HashMap<i64, BigInt>>>,
}

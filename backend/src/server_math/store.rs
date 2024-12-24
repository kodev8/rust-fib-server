use redis::{Client, Commands, Connection, RedisError};
use num_bigint::BigInt;
use std::str::FromStr;
use std::collections::HashMap;

pub trait Store {
    fn get(&mut self, key: i64) -> Result<Option<BigInt>, String>;
    fn set(&mut self, key: i64, value: &BigInt) -> Result<(), String>;
    fn contains_key(&mut self, key: i64) -> Result<bool, String>;
}

// In-memory implementation for unit tests
pub struct HashMapStore {
    store: HashMap<i64, BigInt>,
}

impl HashMapStore {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }

    }
}

impl Store for HashMapStore {
    fn get(&mut self, key: i64) -> Result<Option<BigInt>, String> {
        Ok(self.store.get(&key).cloned())
    }

    fn set(&mut self, key: i64, value: &BigInt) -> Result<(), String> {
        self.store.insert(key, value.clone());
        Ok(())
    }

    fn contains_key(&mut self, key: i64) -> Result<bool, String> {
        Ok(self.store.contains_key(&key))
    }
}

// Redis implementation for production and integration tests
pub struct RedisStore {
    conn: Connection,
    prefix: String,
}

impl RedisStore {
    pub fn new(redis_url: &str, prefix: &str) -> Result<Self, RedisError> {
        let client = Client::open(redis_url)?;
        let conn = client.get_connection()?;
        Ok(RedisStore {
            conn,
            prefix: prefix.to_string(),
        })
    }

    fn get_key(&self, key: i64) -> String {
        format!("{}:{}", self.prefix, key)
    }

    #[allow(dead_code, dependency_on_unit_never_type_fallback)]
    pub fn clear_prefix(&mut self) -> Result<(), RedisError> {
        let pattern = format!("{}:*", self.prefix);
        let keys: Vec<String> = self.conn.keys(pattern)?;
        if !keys.is_empty() {
            self.conn.del(keys)?;
        }
        Ok(())
    }
}

impl Store for RedisStore {
    fn get(&mut self, key: i64) -> Result<Option<BigInt>, String> {
        let redis_key = self.get_key(key);
        self.conn.get(&redis_key)
            .map_err(|e| e.to_string())
            .and_then(|value: Option<String>| {
                value.map(|v| BigInt::from_str(&v)
                    .map_err(|e| e.to_string()))
                    .transpose()
            })
    }

    fn set(&mut self, key: i64, value: &BigInt) -> Result<(), String> {
        let redis_key = self.get_key(key);
        self.conn.set(redis_key, value.to_string())
            .map_err(|e| e.to_string())
    }

    fn contains_key(&mut self, key: i64) -> Result<bool, String> {
        let redis_key = self.get_key(key);
        self.conn.exists(redis_key)
            .map_err(|e| e.to_string())
    }
}

impl<T: Store + ?Sized> Store for Box<T> {
    fn get(&mut self, key: i64) -> Result<Option<BigInt>, String> {
        (**self).get(key)
    }

    fn set(&mut self, key: i64, value: &BigInt) -> Result<(), String> {
        (**self).set(key, value)
    }

    fn contains_key(&mut self, key: i64) -> Result<bool, String> {
        (**self).contains_key(key)
    }
}
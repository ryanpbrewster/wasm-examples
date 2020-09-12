use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn allow(request: &str) -> bool {
    let request: Request = match serde_json::from_str(&request) {
        Ok(value) => value,
        Err(_) => return false,
    };
    request.params.is_empty() && request.auth.uid.chars().zip(request.auth.uid.chars().rev()).all(|(a, b)| a == b)
}

#[derive(Deserialize)]
struct Request {
    auth: Auth,
    #[serde(default)]
    params: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct Auth {
    uid: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_allow() {
        assert!(allow(r#"{"auth":{"uid":"hannah"}}"#));
    }

    #[test]
    fn wrong_uid() {
        assert!(!allow(r#"{"auth":{"uid":"foo"}}"#));
    }

    #[test]
    fn wrong_params() {
        assert!(!allow(r#"{"auth":{"uid":"hannah"},"params":{"a":4}}"#));
    }
}

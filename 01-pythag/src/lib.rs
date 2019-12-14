use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_pythag(a: i32, b: i32, c: i32) -> bool {
    a * a + b * b == c * c
}

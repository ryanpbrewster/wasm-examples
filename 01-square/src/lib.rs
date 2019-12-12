use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn square(n: i32) -> i32 {
    n * n
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reverse(mut xs: Vec<i32>) -> Vec<i32> {
    xs.reverse();
    xs
}

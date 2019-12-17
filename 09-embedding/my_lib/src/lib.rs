use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum_of_squares(n: i32) -> i32 {
  let mut sum = 0;
  let mut i = 1;
  while i <= n {
    sum += i * i;
    i += 1;
  }
  sum
}

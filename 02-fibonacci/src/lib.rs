use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibrec(n: i32) -> i32 {
  if n <= 1 {
    n
  } else {
    fibrec(n - 2) + fibrec(n - 1)
  }
}

#[wasm_bindgen]
pub fn fibiter(n: i32) -> i32 {
  let mut a = 1;
  let mut b = 0;
  for _ in 1 .. n {
    let t = a;
    a = a + b;
    b = t;
  }
  a
}

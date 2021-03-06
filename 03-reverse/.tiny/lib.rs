use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn reverse(mut xs: Vec<i32>) -> Vec<i32> {
    xs.reverse();
    xs
}








#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn smoke_test() {
    assert_eq!(reverse(vec![3, 1, 4, 1, 5]), vec![5, 1, 4, 1, 3]);
  }
}

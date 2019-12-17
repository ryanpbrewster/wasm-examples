use wasmtime::*;

fn main() {
  let engine = HostRef::new(Engine::default());
  let store = HostRef::new(Store::new(&engine));

  let raw_bytes = std::fs::read("my_lib.wasm").unwrap();
  let module = HostRef::new(Module::new(&store, &raw_bytes).unwrap());
  let instance = Instance::new(&store, &module, &[]).unwrap();

  let sum_of_squares = instance.find_export_by_name("sum_of_squares").unwrap().func().unwrap();

  let result = sum_of_squares.borrow().call(&[Val::I32(100)]).unwrap();
  println!("{:?}", result);
}

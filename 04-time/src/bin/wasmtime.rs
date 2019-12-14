use std::time::SystemTime;
fn main() {
  let unix_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
  println!("{}", unix_timestamp);
}

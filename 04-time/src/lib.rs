use std::time::SystemTime;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn unix_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("could not read system clock")
        .as_secs() as u32
}

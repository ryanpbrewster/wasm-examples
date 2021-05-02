use prost::Message;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn entrypoint(request: &[u8]) -> i32 {
    let my_input = match proto::MyInput::decode(request) {
        Ok(v) => v,
        Err(_) => return proto::Outcome::Reject.into(),
    };
    process(my_input).into()
}

fn process(request: proto::MyInput) -> proto::Outcome {
    if request.my_i32 > 42 {
        return proto::Outcome::Reject;
    }
    proto::Outcome::Accept
}

mod proto {
    include!(concat!(env!("OUT_DIR"), "/rpb.example.rs"));
}

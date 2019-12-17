use chrono::{SecondsFormat, TimeZone, Utc};
use std::time::SystemTime;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pretty_time() -> String {
    prettify(unix_timestamp())
}

fn prettify(seconds: i64) -> String {
    Utc.timestamp(seconds, /* nanos = */ 0)
        .to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("could not read system clock")
        .as_secs() as i64
}


















#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test() {
        assert_eq!(prettify(1_234_567_890), "2009-02-13T23:31:30Z");
    }
}

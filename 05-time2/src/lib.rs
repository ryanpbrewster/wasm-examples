use chrono::{SecondsFormat, TimeZone, Utc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f64;
}

#[wasm_bindgen]
pub fn pretty_time() -> String {
    prettify(unix_timestamp())
}

fn prettify(t: i64) -> String {
    Utc.timestamp(t, 0)
        .to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn unix_timestamp() -> i64 {
  now() as i64 / 1_000
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test() {
        assert_eq!(prettify(1_234_567_890), "2009-02-13T23:31:30Z");
    }
}

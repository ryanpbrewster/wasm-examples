use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn add_list_item(s: &str);
}

#[wasm_bindgen]
pub fn do_wasm_stuff(n: i32) {
    let v = format!("{}", nth_prime(n));
    add_list_item(&v);
}

fn nth_prime(n: i32) -> i32 {
    let mut i = 1;
    for _ in 0..=n {
        i += 1;
        while !is_prime(i) {
            i += 1;
        }
    }
    i
}

fn is_prime(i: i32) -> bool {
    let mut j = 2;
    while j * j <= i {
        if i % j == 0 {
            return false;
        }
        j += 1;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_prime_test() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));

        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(!is_prime(11 * 13));
    }

    #[test]
    fn nth_prime_test() {
        assert_eq!(nth_prime(0), 2);
        assert_eq!(nth_prime(1), 3);
        assert_eq!(nth_prime(2), 5);
        assert_eq!(nth_prime(3), 7);
        assert_eq!(nth_prime(4), 11);
    }
}

use nom::{
    bytes::complete::take_while,
    character::complete::multispace1,
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    IResult,
};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(s: String) -> String {
    let vs = parse_input(&s);
    format!("{:?}", vs)
}

fn parse_input(input: &str) -> Vec<i32> {
    match all_consuming(separated_list1(multispace1, i32_parser))(input) {
        Ok((_, vs)) => vs,
        Err(_) => vec![],
    }
}

fn i32_parser(input: &str) -> IResult<&str, i32> {
    map_res(
        take_while(|c: char| c == '+' || c == '-' || c.is_digit(10)),
        |s: &str| s.parse(),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test() {
        assert_eq!(parse("foo".to_owned()), "foo");
    }
}

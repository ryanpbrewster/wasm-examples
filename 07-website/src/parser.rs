use crate::model::{Expression, Identifier, Literal};

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use pest::error::LineColLocation;
use std::convert::TryFrom;
use std::fmt;

#[derive(Parser)]
#[grammar = "cel.pest"]
struct CelParser;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Pest(LineColLocation),
    IllegalInt(String),
    IllegalFloat(String),
}

impl<T: fmt::Debug> From<pest::error::Error<T>> for ParseError {
    fn from(err: pest::error::Error<T>) -> Self {
        ParseError::Pest(err.line_col)
    }
}
impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError::IllegalInt(format!("{}", err))
    }
}
impl From<std::num::ParseFloatError> for ParseError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParseError::IllegalFloat(format!("{}", err))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Pest(LineColLocation::Pos(pos)) => {
                write!(f, "parse error @ L{}:{}", pos.0, pos.1)
            }
            ParseError::Pest(LineColLocation::Span(start, _end)) => {
                write!(f, "parse error @ L{}:{}", start.0, start.1)
            }
            ParseError::IllegalInt(msg) => write!(f, "illegal integer: {}", msg),
            ParseError::IllegalFloat(msg) => write!(f, "illegal integer: {}", msg),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse(input: &str) -> ParseResult<Expression> {
    let mut parsed = CelParser::parse(Rule::TopLevel, input)?;
    extract_top_level(parsed.next().unwrap())
}

fn extract_top_level(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::TopLevel);
    let mut pairs = pair.into_inner();

    let mut bindings = vec![];
    let body = loop {
        let p = pairs.next().unwrap();
        match p.as_rule() {
            Rule::LetBinding => {
                bindings.push(extract_binding(p)?);
            }
            _ => break extract_expression(p)?,
        }
    };

    Ok(bindings
        .into_iter()
        .rfold(body, |expr, (id, value)| Expression::LetBinding {
            id,
            value: Box::new(value),
            body: Box::new(expr),
        }))
}

fn extract_binding(pair: Pair<Rule>) -> ParseResult<(Identifier, Expression)> {
    assert_eq!(pair.as_rule(), Rule::LetBinding);
    let mut pairs = pair.into_inner();
    let id = extract_identifier(pairs.next().unwrap());
    let value = extract_expression(pairs.next().unwrap())?;
    Ok((id, value))
}

fn extract_expression(pair: Pair<Rule>) -> ParseResult<Expression> {
    match pair.as_rule() {
        Rule::Ternary => extract_ternary(pair),
        Rule::Disjunction => extract_disjunction(pair),
        _ => unreachable!(),
    }
}

fn extract_ternary(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Ternary);
    let mut pairs = pair.into_inner();
    let condition = extract_disjunction(pairs.next().unwrap())?;
    let true_branch = extract_expression(pairs.next().unwrap())?;
    let false_branch = extract_expression(pairs.next().unwrap())?;
    Ok(Expression::Ternary {
        condition: Box::new(condition),
        true_branch: Box::new(true_branch),
        false_branch: Box::new(false_branch),
    })
}

fn extract_disjunction(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Disjunction);
    let mut exprs: Vec<Expression> = pair
        .into_inner()
        .map(extract_conjunction)
        .collect::<ParseResult<_>>()?;
    if exprs.len() == 1 {
        Ok(exprs.swap_remove(0))
    } else {
        Ok(Expression::Or(exprs))
    }
}

fn extract_conjunction(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Conjunction);
    let mut exprs: Vec<Expression> = pair
        .into_inner()
        .map(extract_relation)
        .collect::<ParseResult<_>>()?;
    if exprs.len() == 1 {
        Ok(exprs.swap_remove(0))
    } else {
        Ok(Expression::And(exprs))
    }
}

fn extract_relation(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Relation);
    let mut pairs = pair.into_inner();
    let a = extract_addition(pairs.next().unwrap())?;
    let outer = match pairs.next() {
        None => a,
        Some(op) => {
            assert_eq!(op.as_rule(), Rule::RelOp);
            let b = extract_addition(pairs.next().unwrap())?;
            match op.as_str() {
                "==" => Expression::Eq(Box::new(a), Box::new(b)),
                "!=" => Expression::Neq(Box::new(a), Box::new(b)),
                "<" => Expression::Lt(Box::new(a), Box::new(b)),
                "<=" => Expression::Lte(Box::new(a), Box::new(b)),
                ">=" => Expression::Gte(Box::new(a), Box::new(b)),
                ">" => Expression::Gt(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }
    };
    Ok(outer)
}

fn extract_addition(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Addition);
    let mut pairs = pair.into_inner();
    let mut a = extract_multiplication(pairs.next().unwrap())?;
    while let Some(op) = pairs.next() {
        assert_eq!(op.as_rule(), Rule::AddOp);
        let b = extract_multiplication(pairs.next().unwrap())?;
        a = match op.as_str() {
            "+" => Expression::Add(Box::new(a), Box::new(b)),
            "-" => Expression::Sub(Box::new(a), Box::new(b)),
            _ => unreachable!(),
        }
    }
    Ok(a)
}

fn extract_multiplication(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Multiplication);
    let mut pairs = pair.into_inner();
    let mut a = extract_unary(pairs.next().unwrap())?;
    while let Some(op) = pairs.next() {
        assert_eq!(op.as_rule(), Rule::MulOp);
        let b = extract_unary(pairs.next().unwrap())?;
        a = match op.as_str() {
            "*" => Expression::Mul(Box::new(a), Box::new(b)),
            "/" => Expression::Div(Box::new(a), Box::new(b)),
            "%" => Expression::Mod(Box::new(a), Box::new(b)),
            _ => unreachable!(),
        }
    }
    Ok(a)
}

fn extract_unary(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Unary);
    let mut pairs = pair.into_inner();
    let a = pairs.next().unwrap();
    match a.as_rule() {
        Rule::Member => extract_member(a),
        Rule::FunctionCall => extract_function_call(a),
        Rule::UnaryOp => {
            assert_eq!(a.as_rule(), Rule::UnaryOp);
            match a.as_str() {
                "-" => Ok(Expression::Neg(Box::new(extract_unary(
                    pairs.next().unwrap(),
                )?))),
                "!" => Ok(Expression::Not(Box::new(extract_unary(
                    pairs.next().unwrap(),
                )?))),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn extract_member(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Member);
    let mut pairs = pair.into_inner();
    let mut a = extract_operand(pairs.next().unwrap())?;

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::MethodCall => {
                let (id, args) = extract_method_call(pair)?;
                a = Expression::Method(Box::new(a), id, args);
            }
            Rule::MemberRef => {
                let id = extract_member_ref(pair);
                a = Expression::Member(Box::new(a), id);
            }
            Rule::Index => {
                let (id, args) = extract_index(pair)?;
                a = Expression::Method(Box::new(a), id, vec![args]);
            }
            _ => unreachable!(),
        };
    }

    Ok(a)
}

fn extract_function_call(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::FunctionCall);
    let mut pairs = pair.into_inner();
    let id = extract_identifier(pairs.next().unwrap());
    let args = extract_args(pairs.next().unwrap())?;
    Ok(Expression::FunctionCall(id, args))
}

fn extract_operand(pair: Pair<Rule>) -> ParseResult<Expression> {
    assert_eq!(pair.as_rule(), Rule::Operand);
    let a = pair.into_inner().next().unwrap();
    match a.as_rule() {
        Rule::Literal => Ok(Expression::Lit(extract_literal(a)?)),
        Rule::Identifier => Ok(Expression::Binding(extract_identifier(a))),
        Rule::FunctionCall => extract_function_call(a),
        _ => extract_expression(a),
    }
}

fn extract_method_call(pair: Pair<Rule>) -> ParseResult<(Identifier, Vec<Expression>)> {
    assert_eq!(pair.as_rule(), Rule::MethodCall);
    let mut pairs = pair.into_inner();
    Ok((
        extract_identifier(pairs.next().unwrap()),
        extract_args(pairs.next().unwrap())?,
    ))
}

fn extract_member_ref(pair: Pair<Rule>) -> Identifier {
    assert_eq!(pair.as_rule(), Rule::MemberRef);
    extract_identifier(pair.into_inner().next().unwrap())
}

fn extract_index(pair: Pair<Rule>) -> ParseResult<(Identifier, Expression)> {
    assert_eq!(pair.as_rule(), Rule::Index);
    let mut pairs = pair.into_inner();
    Ok((
        Identifier("get".to_owned()),
        extract_expression(pairs.next().unwrap())?,
    ))
}

fn extract_identifier(pair: Pair<Rule>) -> Identifier {
    assert_eq!(pair.as_rule(), Rule::Identifier);
    pair.as_str().parse().expect("parse identifier")
}

fn extract_args(pair: Pair<Rule>) -> ParseResult<Vec<Expression>> {
    assert_eq!(pair.as_rule(), Rule::Args);
    pair.into_inner().map(extract_expression).collect()
}

fn extract_literal(pair: Pair<Rule>) -> ParseResult<Literal> {
    assert_eq!(pair.as_rule(), Rule::Literal);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::StringLiteral => Ok(Literal::String(extract_string(pair))),
        Rule::BytesLiteral => Ok(Literal::Bytes(extract_bytes(pair))),
        Rule::FloatLiteral => Ok(Literal::F64(pair.as_str().replace("_", "").parse()?)),
        Rule::IntLiteral => Ok(Literal::I64(pair.as_str().replace("_", "").parse()?)),
        Rule::ListLiteral => extract_list(pair),
        Rule::MapLiteral => extract_map(pair),
        Rule::BoolLiteral => Ok(Literal::Bool(pair.as_str().parse().unwrap())),
        Rule::NullLiteral => Ok(Literal::Null),
        _ => unreachable!(),
    }
}

fn extract_string(pair: Pair<Rule>) -> String {
    assert_eq!(pair.as_rule(), Rule::StringLiteral);
    let mut buf = String::new();
    for p in pair.into_inner() {
        match unescape_sequence(&p) {
            Unescaped::Byte(b) => buf.push(b as char),
            Unescaped::Unicode(ch) => buf.push(ch),
        };
    }
    buf
}

fn extract_bytes(pair: Pair<Rule>) -> Vec<u8> {
    assert_eq!(pair.as_rule(), Rule::BytesLiteral);
    let mut buf = Vec::new();
    for p in pair.into_inner() {
        match unescape_sequence(&p) {
            Unescaped::Byte(b) => buf.push(b),
            Unescaped::Unicode(ch) => buf.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()),
        };
    }
    buf
}

enum Unescaped {
    Byte(u8),
    Unicode(char),
}
fn unescape_sequence(pair: &Pair<Rule>) -> Unescaped {
    match pair.as_rule() {
        Rule::CharLiteral => Unescaped::Unicode(pair.as_str().chars().next().unwrap()),
        Rule::Escape => {
            let s = &pair.as_str()[1..];
            match &s[..1] {
                "t" => Unescaped::Byte(b'\t'),
                "n" => Unescaped::Byte(b'\n'),
                "\"" => Unescaped::Byte(b'"'),
                "x" => Unescaped::Byte(u8::from_str_radix(&s[1..], 16).unwrap()),
                "u" => Unescaped::Unicode(
                    char::try_from(u32::from_str_radix(&s[1..], 16).unwrap()).unwrap(),
                ),
                "0" | "1" | "2" | "3" => Unescaped::Byte(u8::from_str_radix(s, 8).unwrap()),
                _ => unreachable!("unexpected string literal {}", s),
            }
        }
        _ => unreachable!(),
    }
}

fn extract_list(pair: Pair<Rule>) -> ParseResult<Literal> {
    assert_eq!(pair.as_rule(), Rule::ListLiteral);
    let mut vs = Vec::new();
    for p in pair.into_inner() {
        vs.push(extract_expression(p)?);
    }
    Ok(Literal::List(vs))
}

fn extract_map(pair: Pair<Rule>) -> ParseResult<Literal> {
    assert_eq!(pair.as_rule(), Rule::MapLiteral);
    let mut fields = Vec::new();
    for p in pair.into_inner() {
        fields.push(extract_map_field(p)?);
    }
    Ok(Literal::Map(fields))
}

fn extract_map_field(pair: Pair<Rule>) -> ParseResult<(Expression, Expression)> {
    assert_eq!(pair.as_rule(), Rule::MapField);
    let mut pairs = pair.into_inner();
    Ok((
        extract_expression(pairs.next().unwrap())?,
        extract_expression(pairs.next().unwrap())?,
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Literal;
    use std::any::Any;

    fn assert_valid(input: &str) {
        parse(input).expect("failed to parse");
    }

    fn assert_invalid(input: &str) {
        let parsed = parse(input);
        assert!(parsed.is_err(), "{} was accepted as {:?}", input, parsed);
    }

    fn literal(x: &dyn Any) -> Expression {
        if let Some(&s) = x.downcast_ref::<&str>() {
            return Expression::Lit(Literal::String(String::from(s)));
        }
        if let Some(&b) = x.downcast_ref::<&[u8]>() {
            return Expression::Lit(Literal::Bytes(b.to_vec()));
        }
        unimplemented!("literal of type {:?}", x.type_id())
    }

    #[test]
    fn cel_valid() {
        assert_valid("22 * (4 + 15)");
        assert_valid("22 * -4");
        assert_valid("!false");
    }

    #[test]
    fn float_literals() {
        assert_valid("3.1415926");
        assert_valid("1_024__.1_4_1_5_____");
        assert_invalid(".1415926");
        assert_invalid("3.");
        assert_invalid("3._0");
    }

    #[test]
    fn float_literal_overflow() {
        assert_eq!(
            parse("9999999999999999999999999.0"),
            Ok(Expression::Lit(Literal::F64(1e25))),
        );
    }

    #[test]
    fn null_literal() {
        assert_valid("null");
    }

    #[test]
    fn int_literals() {
        assert_valid("1");
        assert_valid("31415");
        assert_valid("1_000_000_000");
    }

    #[test]
    fn int_literal_overflow() {
        assert_eq!(
            parse("9999999999999999999999999"),
            Err(ParseError::IllegalInt(
                "number too large to fit in target type".to_owned()
            ))
        );
    }

    #[test]
    fn cel_smoke() {
        let input = "22 * (4 + 15)";
        assert_eq!(
            parse(input),
            Ok(Expression::Mul(
                Box::new(Expression::Lit(Literal::I64(22))),
                Box::new(Expression::Add(
                    Box::new(Expression::Lit(Literal::I64(4))),
                    Box::new(Expression::Lit(Literal::I64(15))),
                ))
            ))
        );
    }

    #[test]
    fn cel_list() {
        let input = "[0, false || true]";
        assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::List(vec![
                Expression::Lit(Literal::I64(0)),
                Expression::Or(vec![
                    Expression::Lit(Literal::Bool(false)),
                    Expression::Lit(Literal::Bool(true)),
                ])
            ])))
        );
    }

    #[test]
    fn valid_list() {
        assert_valid(r#" [] "#); // empty
        assert_valid(r#" [x, y, z] "#); // bindings
        assert_valid(r#" [ "a", "b", "c",  ] "#); // trailing comma
        assert_valid(r#" [[[[[0]]]]] "#); // nested
    }

    #[test]
    fn valid_maps() {
        assert_valid(r#" { "foo": "bar" } "#); // string literals
        assert_valid(r#" { foo: bar } "#); // bindings
        assert_valid(r#" { a: b, c: d, e: f } "#); // multiple fields
        assert_valid(r#" {} "#); // empty
        assert_valid(
            r#" {
          "a": "b",
          "p": "q"
        } "#,
        ); // multi-line
        assert_valid(
            r#" {
          "a": "b",
          "p": "q",
        } "#,
        ); // trailing comma
    }

    #[test]
    fn cel_string() {
        let input = r#""asdf""#;
        assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::String(String::from("asdf"))))
        );
    }

    #[test]
    fn valid_string_literals() {
        assert_valid(r#" "asdf" "#);
        assert_valid(r#" 'asdf' "#);
        assert_valid(r#" '¢' "#);
    }

    #[test]
    fn cel_escaped_quote_string() {
        let input = r#""as\"df""#;
        assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::String(String::from("as\"df"))))
        );
    }

    #[test]
    fn invalid_octal_escapes() {
        assert_invalid(r#" "\0" "#);
        assert_invalid(r#" "\7" "#);
        assert_invalid(r#" "\07" "#);
        assert_invalid(r#" "\77" "#);
        assert_invalid(r#" "\8" "#);
        assert_invalid(r#" "\378" "#);
        assert_invalid(r#" "\400" "#);
    }

    #[test]
    fn valid_octal_escapes() {
        assert_eq!(parse(r#" "\000" "#).unwrap(), literal(&"\u{0000}"));
        assert_eq!(parse(r#" "\007" "#).unwrap(), literal(&"\u{0007}"));
        assert_eq!(parse(r#" "\377" "#).unwrap(), literal(&"\u{00FF}"));
    }

    #[test]
    fn valid_hex_escapes() {
        assert_eq!(parse(r#" "\x00" "#).unwrap(), literal(&"\u{0000}"));
        assert_eq!(parse(r#" "\xFF" "#).unwrap(), literal(&"\u{00FF}"));
    }

    #[test]
    fn valid_unicode_escapes() {
        assert_eq!(parse(r#" "\u0000" "#).unwrap(), literal(&"\u{0000}"));
        assert_eq!(parse(r#" "\u00FF" "#).unwrap(), literal(&"\u{00FF}"));
        assert_eq!(parse(r#" "\uFF00" "#).unwrap(), literal(&"\u{FF00}"));
        assert_eq!(parse(r#" "\uFFFF" "#).unwrap(), literal(&"\u{FFFF}"));
    }

    #[test]
    fn valid_bytes() {
        assert_eq!(parse(r#" b"asdf" "#).unwrap(), literal(&"asdf".as_bytes()));
    }

    #[test]
    fn method_call() {
        assert_valid(r#" [1, 2, 3].len() "#);
        assert_valid(r#" 42.pow(42) "#);
        assert_valid(r#" ([1] + [2]).len() "#);
        assert_valid(r#" ([1] + [2]).len().pow(2) "#);
        assert_valid(r#" ([1] + [2]).foo.bar.baz(1,2,3).length.asdf("asdf") "#);
    }

    #[test]
    fn member_access() {
        assert_valid(r#" foo "#);
        assert_valid(r#" foo.bar.baz "#);
    }

    #[test]
    fn multi_conjunction() {
        assert_valid(r#" true && true && true && true "#);
    }

    #[test]
    fn multi_disjunction() {
        assert_valid(r#" true || true || true || true "#);
    }

    #[test]
    fn relations() {
        assert_valid(r#" 0 < 1 && 1 <= 2 && 3 == 3 && 4 >= 3 && 4 > 3 && 0 != 0 "#);
    }

    #[test]
    fn let_binding_smoke() {
        assert_valid(r#" let x = 42; x "#);
    }

    #[test]
    fn let_binding_no_expression() {
        assert_invalid(r#" let x = 42; "#);
    }

    #[test]
    fn let_rebinding() {
        assert_valid(r#" let x = 42; let x = x*x; x "#);
    }

    #[test]
    fn ternary_operator() {
        assert_valid(r#" true ? 1 : 2 "#);
        assert_valid(r#" foo == bar || baz ? 1 : 2 "#);
        assert_valid(r#" a ? b : c ? d : e "#); // chianing is fine
        assert_valid(r#" a?b:c "#); // whitespace is optional
        assert_valid(r#" ( foo ? true : bar ? true : true) "#); // inside parentheses
    }

    #[test]
    fn ternary_operator_inside_map_literal() {
        assert_valid(r#" { true ? "a" : "b" : "foo"  } "#); //  evaluates to { "a": "foo" }
        assert_valid(r#" { "a" : true ? "foo" : bar } "#); //  evaluates to { "a": "foo" }
    }

    #[test]
    fn function_call() {
        assert_valid(r#"evaluate("SQL", {})"#);
    }

    #[test]
    fn function_call_then_method() {
        assert_valid(r#"evaluate("SQL", {}).len()"#);
    }

    #[test]
    fn map_get() {
        assert_valid(r#"{'a': 'a'}['a']"#);
    }
}

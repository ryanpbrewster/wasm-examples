use std::collections::HashMap;

use crate::model::{Error, EvalResult, Expression, Identifier, Op, Value};
use std::cmp::Ordering;

pub mod walker;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Lit(Value),
    TypeError(Op),
    MakeList(usize),
    MakeMap(usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Not,
    Or,
    And,
    Eq,
    Lt,
    Lte,
    Gte,
    Gt,
    Clone,
    Pop,
    JumpIf(usize),
    JumpIfNot(usize),
    JumpError(usize),
    Jump(usize),
    Abort,
    Member(Identifier),
    Ternary,
}

impl Operation {
    pub fn short(&self) -> String {
        match self {
            Operation::Lit(_) => format!("LIT"),
            Operation::MakeList(_) => format!("MKLIST"),
            Operation::MakeMap(_) => format!("MKMAP"),
            Operation::Add => format!("ADD"),
            Operation::Sub => format!("SUB"),
            Operation::Mul => format!("MUL"),
            Operation::Div => format!("DIV"),
            Operation::Mod => format!("MOD"),
            Operation::Neg => format!("NEG"),
            Operation::Not => format!("NOT"),
            Operation::Or => format!("OR"),
            Operation::And => format!("AND"),
            Operation::Eq => format!("EQ"),
            Operation::Lt => format!("LT"),
            Operation::Lte => format!("LTE"),
            Operation::Gte => format!("GTE"),
            Operation::Gt => format!("GT"),
            Operation::Jump(_) => format!("JMP"),
            Operation::JumpError(_) => format!("JMPERR"),
            Operation::JumpIf(_) => format!("JMPIF"),
            Operation::JumpIfNot(_) => format!("JMPIFN"),
            Operation::Clone => format!("CLONE"),
            Operation::Pop => format!("POP"),
            Operation::TypeError(_) => format!("TYPERR"),
            Operation::Abort => format!("ABRT"),
            Operation::Member(_) => format!("MBR"),
            Operation::Ternary => format!("TERNRY"),
        }
    }

    pub fn tooltip(&self) -> String {
        match self {
            Operation::Lit(v) => format!("pushes the value {} onto the stack", v),
            Operation::MakeList(n) => format!("pop {} items and construct a list from them", n),
            Operation::MakeMap(n) => format!(
                "pop {} items and construct a map from the key-value pairs",
                2 * n
            ),
            Operation::Add => format!("pop 2 items and push their sum"),
            Operation::Sub => format!("pop 2 items and push their difference"),
            Operation::Mul => format!("pop 2 items and push their product"),
            Operation::Div => format!("pop 2 items and divide them"),
            Operation::Mod => format!("pop 2 items and take the modulus"),
            Operation::Neg => format!("pop a number and negate it"),
            Operation::Not => format!("pop a boolean and negate it"),
            Operation::Or => format!("combine two booleans"),
            Operation::And => format!("combine two booleans"),
            Operation::Eq => format!("pop two items and check if they're equal"),
            Operation::Jump(n) => format!("jump {} operations", n),
            Operation::JumpError(n) => format!(
                "peek at the top of the stack; jmp {} operations if it is an error",
                n
            ),
            Operation::JumpIf(n) => format!(
                "peek at the top of the stack; jump {} operations if it is true",
                n
            ),
            Operation::JumpIfNot(n) => format!(
                "peek at the top of the stack; jump {} operations if it is false",
                n
            ),
            Operation::Clone => {
                format!("pop an item and push two copies of it back onto the stack")
            }
            Operation::Pop => format!("pop an item and discard it"),
            Operation::TypeError(op) => {
                format!("pop an item and construct a type-error for {:?}", op)
            }
            Operation::Lt => format!("pop two items and check if the second is < than the first"),
            Operation::Lte => format!("pop two items and check if the second is <= than the first"),
            Operation::Gte => format!("pop two items and check if the second is >= than the first"),
            Operation::Gt => format!("pop two items and check if the second is > than the first"),
            Operation::Abort => {
                format!("abort the program (usually because something isn't implemented)")
            }
            Operation::Member(_) => format!(
                "pop a map and a string off the stack, then fetch the named member from the map"
            ),
            Operation::Ternary => {
                format!("pop two values off the stack, push the original head back on")
            }
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub operations: Vec<Operation>,
    pub pointer: usize,
    pub stack: Vec<EvalResult>,
}

impl Program {
    pub fn step(&mut self) -> bool {
        if self.pointer >= self.operations.len() {
            return false;
        }
        match self.operations[self.pointer] {
            Operation::Lit(ref v) => self.stack.push(Ok(v.clone())),
            Operation::TypeError(ref op) => {
                let a = self.stack.pop().unwrap();
                let err = match a {
                    Ok(v) => Error::InvalidTypeForOperator(v.kind(), op.clone()),
                    Err(e) => e,
                };
                self.stack.push(Err(err));
            }
            Operation::Clone => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a.clone());
                self.stack.push(a);
            }
            Operation::Pop => {
                self.stack.pop();
            }
            Operation::MakeList(n) => {
                let mut acc = Ok(Vec::new());
                for _ in 0..n {
                    let v = self.stack.pop().unwrap();
                    if let Ok(ref mut vs) = acc {
                        match v {
                            Ok(v) => vs.push(v),
                            Err(e) => acc = Err(e),
                        }
                    }
                }
                self.stack.push(acc.map(|mut vs| {
                    vs.reverse();
                    Value::List(vs)
                }));
            }
            Operation::MakeMap(n) => {
                let mut acc = Ok(HashMap::new());
                for _ in 0..n {
                    let k = self.stack.pop().unwrap();
                    let v = self.stack.pop().unwrap();
                    if let Ok(ref mut kvs) = acc {
                        match (k, v) {
                            (Ok(k), Ok(v)) => match k {
                                Value::String(k) => {
                                    if kvs.insert(k.clone(), v).is_some() {
                                        acc = Err(Error::DuplicateMapKey(k));
                                    }
                                }
                                other => acc = Err(Error::InvalidMapKey(other.kind())),
                            },
                            (Err(e), _) | (_, Err(e)) => acc = Err(e),
                        }
                    }
                }
                self.stack.push(acc.map(|kvs| Value::Map(kvs)));
            }
            Operation::Add => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(a), Ok(b)) => eval_add(a, b),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Sub => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(a), Ok(b)) => eval_sub(a, b),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Mul => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(a), Ok(b)) => eval_mul(a, b),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Div => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(a), Ok(b)) => eval_div(a, b),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Mod => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(a), Ok(b)) => eval_mod(a, b),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Neg => {
                let a = self.stack.pop().unwrap();
                let result = match a {
                    Ok(Value::I64(n)) => Ok(Value::I64(-n)),
                    Ok(Value::F64(x)) => Ok(Value::F64(-x)),
                    Ok(other) => Err(Error::InvalidTypeForOperator(other.kind(), Op::Neg)),
                    Err(e) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Not => {
                let a = self.stack.pop().unwrap();
                let result = match a {
                    Ok(Value::Bool(b)) => Ok(Value::Bool(!b)),
                    Ok(other) => Err(Error::InvalidTypeForOperator(other.kind(), Op::Not)),
                    Err(e) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Or => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(Value::Bool(true)), _) | (_, Ok(Value::Bool(true))) => {
                        Ok(Value::Bool(true))
                    }
                    (Ok(Value::Bool(false)), Ok(Value::Bool(false))) => Ok(Value::Bool(false)),
                    (Ok(a), Ok(b)) => {
                        Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Or))
                    }
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::And => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(Value::Bool(false)), _) | (_, Ok(Value::Bool(false))) => {
                        Ok(Value::Bool(false))
                    }
                    (Ok(Value::Bool(true)), Ok(Value::Bool(true))) => Ok(Value::Bool(true)),
                    (Ok(a), Ok(b)) => {
                        Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::And))
                    }
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Eq => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = match (a, b) {
                    (Ok(a), Ok(b)) => Ok(Value::Bool(a == b)),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                };
                self.stack.push(result);
            }
            Operation::Lt => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(eval_cmp(a, b, Op::Lt));
            }
            Operation::Lte => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(eval_cmp(a, b, Op::Lte));
            }
            Operation::Gte => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(eval_cmp(a, b, Op::Gte));
            }
            Operation::Gt => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(eval_cmp(a, b, Op::Gt));
            }
            Operation::Jump(n) => {
                self.pointer += n;
            }
            Operation::JumpError(n) => {
                let condition = self.stack.last().unwrap();
                if condition.is_err() {
                    self.pointer += n;
                }
            }
            Operation::JumpIf(n) => {
                let condition = self.stack.pop().unwrap();
                match condition {
                    Ok(Value::Bool(a)) => {
                        if a {
                            self.pointer += n;
                        }
                        self.stack.push(condition.clone());
                    }
                    Ok(other) => {
                        self.stack
                            .push(Err(Error::InvalidTypeForOperator(other.kind(), Op::Jump)));
                    }
                    Err(_) => {
                        self.stack.push(condition.clone());
                    }
                }
            }
            Operation::JumpIfNot(n) => {
                let condition = self.stack.pop().unwrap();
                match condition {
                    Ok(Value::Bool(a)) => {
                        if !a {
                            self.pointer += n;
                        }
                        self.stack.push(condition.clone());
                    }
                    Ok(other) => {
                        self.stack
                            .push(Err(Error::InvalidTypeForOperator(other.kind(), Op::Jump)));
                    }
                    Err(_) => {
                        self.stack.push(condition.clone());
                    }
                }
            }
            Operation::Abort => {
                self.stack.clear();
                self.stack.push(Err(Error::Aborted));
                self.pointer = self.operations.len() - 1;
            }
            Operation::Member(_) => {
                let id = self.stack.pop().unwrap();
                let operand = self.stack.pop().unwrap();
                let result = match (operand, id) {
                    (Err(e), _) | (_, Err(e)) => Err(e),
                    (Ok(Value::Map(mut kvs)), Ok(Value::String(s))) => match kvs.remove(&s) {
                        None => Err(Error::NoSuchMember(Identifier::new(&s))),
                        Some(v) => Ok(v),
                    },
                    (Ok(operand), Ok(id)) => Err(Error::InvalidTypesForOperator(
                        operand.kind(),
                        id.kind(),
                        Op::Member,
                    )),
                };
                self.stack.push(result);
            }
            Operation::Ternary => {
                let result = self.stack.pop().unwrap();
                self.stack.pop();
                self.stack.push(result);
            }
        }
        self.pointer += 1;
        true
    }

    pub fn run(&mut self) -> &EvalResult {
        while self.step() {}
        assert_eq!(self.stack.len(), 1, "valid programs always terminate with exactly one value on the stack, this one has {:?}", self.stack);
        self.stack.first().unwrap()
    }
}

pub fn compile(expr: Expression) -> Program {
    Program {
        operations: walker::linearize(expr),
        pointer: 0,
        stack: Vec::new(),
    }
}

fn eval_add(a: Value, b: Value) -> EvalResult {
    match (a, b) {
        (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
        (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
        (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Plus)),
    }
}

fn eval_sub(a: Value, b: Value) -> EvalResult {
    match (a, b) {
        (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a - b)),
        (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a - b)),
        (a, b) => Err(Error::InvalidTypesForOperator(
            a.kind(),
            b.kind(),
            Op::Minus,
        )),
    }
}

fn eval_mul(a: Value, b: Value) -> EvalResult {
    match (a, b) {
        (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a * b)),
        (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a * b)),
        (a, b) => Err(Error::InvalidTypesForOperator(
            a.kind(),
            b.kind(),
            Op::Times,
        )),
    }
}

fn eval_div(a: Value, b: Value) -> EvalResult {
    match (a, b) {
        (Value::I64(a), Value::I64(b)) => {
            if b != 0 {
                Ok(Value::I64(a / b))
            } else {
                Err(Error::DivisionByZero)
            }
        }
        (Value::F64(a), Value::F64(b)) => {
            if b != 0.0 {
                Ok(Value::F64(a / b))
            } else {
                Err(Error::DivisionByZero)
            }
        }
        (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Div)),
    }
}

fn eval_mod(a: Value, b: Value) -> EvalResult {
    match (a, b) {
        (Value::I64(a), Value::I64(b)) => {
            if b != 0 {
                Ok(Value::I64(a % b))
            } else {
                Err(Error::DivisionByZero)
            }
        }
        (Value::F64(a), Value::F64(b)) => {
            if b != 0.0 {
                Ok(Value::F64(a % b))
            } else {
                Err(Error::DivisionByZero)
            }
        }
        (a, b) => Err(Error::InvalidTypesForOperator(a.kind(), b.kind(), Op::Mod)),
    }
}

fn eval_cmp(a: EvalResult, b: EvalResult, op: Op) -> EvalResult {
    let ord = match (a?, b?) {
        (Value::I64(a), Value::I64(b)) => Ok(a.cmp(&b)),
        (a, b) => Err(Error::InvalidTypesForOperator(
            a.kind(),
            b.kind(),
            op.clone(),
        )),
    };
    let result = match ord? {
        Ordering::Less => op == Op::Lt || op == Op::Lte,
        Ordering::Equal => op == Op::Lte || op == Op::Gte,
        Ordering::Greater => op == Op::Gte || op == Op::Gt,
    };
    Ok(Value::Bool(result))
}

#[cfg(test)]
mod test {
    use crate::model::Kind;
    use crate::parser::parse;

    use super::*;

    #[test]
    fn simple_add() {
        let mut program = compile(parse(r#" 1 + 2 "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::I64(1 + 2)));
    }

    #[test]
    fn simple_sub() {
        let mut program = compile(parse(r#" 3 - 2 "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::I64(3 - 2)));
    }

    #[test]
    fn simple_mul() {
        let mut program = compile(parse(r#" 2 * 3 "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::I64(2 * 3)));
    }

    #[test]
    fn simple_div() {
        let mut program = compile(parse(r#" 6 / 3 "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::I64(6 / 3)));
    }

    #[test]
    fn simple_mod() {
        let mut program = compile(parse(r#" 7 % 3 "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::I64(7 % 3)));
    }

    #[test]
    fn float_mod() {
        let mut program = compile(parse(r#" 7.0 % 3.4 "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::F64(7.0 % 3.4)));
    }

    #[test]
    fn simple_or() {
        let mut program = compile(parse(r#" true || false "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::Bool(true)));
    }

    #[test]
    fn or_err() {
        let mut program = compile(parse(r#" false || "asdf" || false "#).unwrap());
        assert_eq!(
            program.run(),
            &Err(Error::InvalidTypesForOperator(
                Kind::Bool,
                Kind::String,
                Op::Or
            )),
        );
    }

    #[test]
    fn or_err_recovery() {
        let mut program =
            compile(parse(r#" 0 || false || 2 || true || 4 || false || 6 || 7 "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::Bool(true)));
    }

    #[test]
    fn simple_ternary() {
        let mut program = compile(parse(r#" 1 + 1 == 2 ? "okay" : "nope" "#).unwrap());
        assert_eq!(program.run(), &Ok(Value::String("okay".to_owned())),);
    }

    #[test]
    fn ternary_bad_type() {
        let mut program = compile(parse(r#" 1 ? 2 : 3 "#).unwrap());
        assert_eq!(
            program.run(),
            &Err(Error::InvalidTypeForOperator(Kind::I64, Op::Jump))
        );
    }

    #[test]
    fn ternary_err() {
        let mut program = compile(parse(r#" 1 / 0 ? 2 : 3 "#).unwrap());
        assert_eq!(program.run(), &Err(Error::DivisionByZero));
    }

    #[test]
    fn simple_list() {
        let mut program = compile(parse(r#" [1, 2 + 3] "#).unwrap());
        assert_eq!(
            program.run(),
            &Ok(Value::List(vec![Value::I64(1), Value::I64(2 + 3)]))
        );
    }
}

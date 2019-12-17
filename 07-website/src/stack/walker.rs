use crate::model::{Expression, Literal, Value};
use crate::stack::Operation;

pub fn linearize(e: Expression) -> Vec<Operation> {
    let mut walker = Walker::new();
    walker.walk(e);
    walker.0
}

struct Walker(Vec<Operation>);
impl Walker {
    fn new() -> Walker {
        Walker(Vec::new())
    }
    fn walk(&mut self, e: Expression) {
        match e {
            Expression::LetBinding { .. } => {
                self.0.push(Operation::Abort);
            }
            Expression::Ternary {
                condition,
                true_branch,
                false_branch,
            } => {
                self.walk(*condition);
                let mut true_subprogram = linearize(*true_branch);
                let mut false_subprogram = linearize(*false_branch);

                let true_len = true_subprogram.len();
                let false_len = false_subprogram.len();

                self.0.push(Operation::JumpIf(false_len + 2));
                self.0.push(Operation::JumpError(false_len + true_len + 2));
                self.0.append(&mut false_subprogram);
                self.0.push(Operation::Jump(true_len));
                self.0.append(&mut true_subprogram);
                self.0.push(Operation::Ternary);
            }
            Expression::Or(exprs) => {
                let mut iter = exprs.into_iter();
                self.walk(iter.next().unwrap());
                for expr in iter {
                    let mut subprogram = linearize(expr);
                    self.0.push(Operation::JumpIf(subprogram.len() + 1));
                    self.0.append(&mut subprogram);
                    self.0.push(Operation::Or);
                }
            }
            Expression::And(exprs) => {
                let mut iter = exprs.into_iter();
                self.walk(iter.next().unwrap());
                for expr in iter {
                    let mut subprogram = linearize(expr);
                    self.0.push(Operation::JumpIfNot(subprogram.len() + 1));
                    self.0.append(&mut subprogram);
                    self.0.push(Operation::And);
                }
            }
            Expression::Eq(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Eq);
            }
            Expression::Neq(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Eq);
                self.0.push(Operation::Not);
            }
            Expression::Lt(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Lt);
            }
            Expression::Lte(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Lte);
            }
            Expression::Gte(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Gte);
            }
            Expression::Gt(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Gt);
            }
            Expression::Add(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Add);
            }
            Expression::Sub(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Sub);
            }
            Expression::Mul(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Mul);
            }
            Expression::Div(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Div);
            }
            Expression::Mod(a, b) => {
                self.walk(*a);
                self.walk(*b);
                self.0.push(Operation::Mod);
            }
            Expression::Neg(a) => {
                self.walk(*a);
                self.0.push(Operation::Neg);
            }
            Expression::Not(a) => {
                self.walk(*a);
                self.0.push(Operation::Not);
            }
            Expression::Member(operand, id) => {
                self.walk(*operand);
                self.0.push(Operation::Member(id));
            }
            Expression::Method(_operand, _name, _args) => {
                self.0.push(Operation::Abort);
            }
            Expression::Lit(lit) => self.walk_literal(lit),
            Expression::Binding(_id) => {
                self.0.push(Operation::Abort);
            }
            Expression::FunctionCall(_id, _args) => {
                self.0.push(Operation::Abort);
            }
        }
    }

    fn walk_literal(&mut self, lit: Literal) {
        match lit {
            Literal::Null => self.0.push(Operation::Lit(Value::Null)),
            Literal::I64(v) => self.0.push(Operation::Lit(Value::I64(v))),
            Literal::F64(v) => self.0.push(Operation::Lit(Value::F64(v))),
            Literal::Bool(v) => self.0.push(Operation::Lit(Value::Bool(v))),
            Literal::String(v) => self.0.push(Operation::Lit(Value::String(v))),
            Literal::Bytes(v) => self.0.push(Operation::Lit(Value::Bytes(v))),
            Literal::List(vs) => {
                let n = vs.len();
                for v in vs.into_iter() {
                    self.walk(v);
                }
                self.0.push(Operation::MakeList(n));
            }
            Literal::Map(vs) => {
                let n = vs.len();
                for (k, v) in vs.into_iter() {
                    self.walk(k);
                    self.walk(v);
                }
                self.0.push(Operation::MakeMap(n));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse;

    use super::*;

    #[test]
    fn linearize_add() {
        let expr = parse(r#" 1 + 2 "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Lit(Value::I64(1)),
                Operation::Lit(Value::I64(2)),
                Operation::Add,
            ]
        );
    }

    #[test]
    fn linearize_sub() {
        let expr = parse(r#" 1 - 2 "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Lit(Value::I64(1)),
                Operation::Lit(Value::I64(2)),
                Operation::Sub,
            ]
        );
    }

    #[test]
    fn linearize_list() {
        let expr = parse(r#" [1, 2 + 3] "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Lit(Value::I64(1)),
                Operation::Lit(Value::I64(2)),
                Operation::Lit(Value::I64(3)),
                Operation::Add,
                Operation::MakeList(2),
            ]
        );
    }

    #[test]
    fn linearize_list_empty() {
        let expr = parse(r#" [] "#).unwrap();
        assert_eq!(linearize(expr), vec![Operation::MakeList(0),]);
    }

    #[test]
    fn linearize_or_simple() {
        let expr = parse(r#" true || false "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Lit(Value::Bool(true)),
                Operation::JumpIf(2),
                Operation::Lit(Value::Bool(false)),
                Operation::Or,
            ]
        );
    }

    #[test]
    fn linearize_or_many() {
        let expr = parse(r#" 0 || 1 || 2 || 3 "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Lit(Value::I64(0)),
                Operation::JumpIf(2),
                Operation::Lit(Value::I64(1)),
                Operation::Or,
                Operation::JumpIf(2),
                Operation::Lit(Value::I64(2)),
                Operation::Or,
                Operation::JumpIf(2),
                Operation::Lit(Value::I64(3)),
                Operation::Or,
            ]
        );
    }

    #[test]
    fn linearize_ternary() {
        let expr = parse(r#" true ? 1 : 2 "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Lit(Value::Bool(true)),
                Operation::JumpIf(3),
                Operation::JumpError(4),
                Operation::Lit(Value::I64(2)),
                Operation::Jump(1),
                Operation::Lit(Value::I64(1)),
                Operation::Ternary,
            ]
        );
    }
}

use crate::expression::{add, divide, integer, multiply, subtract, Expression, Operator, Value};
use Operator::{Add, Divide, Multiply, Subtract};
use Expression::{BinaryExpression, Assignment, Identifier, ValueExpression};
use std::collections::HashMap;
use std::panic::panic_any;

struct Interpreter<'a> {
    environment: &'a mut HashMap<&'a str, Value>
}

impl<'a> Interpreter<'a> {
    fn interpret(&mut self, expression: Expression<'a>) -> Value {
        match expression {
            BinaryExpression { operator, lhs, rhs } => match operator {
                    Add => self.interpret(*lhs) + self.interpret(*rhs),
                    Subtract => self.interpret(*lhs) - self.interpret(*rhs),
                    Multiply => self.interpret(*lhs) * self.interpret(*rhs),
                    Divide => self.interpret(*lhs) / self.interpret(*rhs),
                },
            Assignment { name, expr } => {
                let value = self.interpret(*expr);
                self.environment.insert(name, value);
                value
            }
            Identifier(name)  => {
                match self.environment.get(name) {
                    Some(ValueExpression(val)) => *val,
                    Some(_) => panic!("unreached process."),
                    None => panic!("{} is undefined.", name)
                }
            }
            ValueExpression(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_10_plus_20_should_work() {
        let mut interpreter = Interpreter { environment: &mut HashMap::new() };
        assert_eq!(interpreter.interpret(add(integer(10), integer(20))), 30)
    }
}
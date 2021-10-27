use crate::ast::{add, divide, integer, multiply, subtract, Environment, Expression, Operator, Program, Value};
use Operator::{Add, Divide, EqualEqual, GreaterOrEqual, GreaterThan, LessOrEqual, LessThan, Multiply, NotEqual, Subtract};
use Expression::{BinaryExpression, BlockExpression, Assignment, Identifier, IfExpression, ValueExpression, WhileExpression};
use std::collections::HashMap;
use std::panic::panic_any;

struct Interpreter<'a> {
    environment: Environment<'a>
}

impl<'a> Interpreter<'a> {
    pub fn call_main(program: Program) -> Value {
        let mut topLevels = program.definitions();
    }
    fn interpret(&mut self, expression: Expression<'a>) -> Value {
        match expression {
            BinaryExpression { operator, lhs, rhs } => match operator {
                    Add => self.interpret(*lhs) + self.interpret(*rhs),
                    Subtract => self.interpret(*lhs) - self.interpret(*rhs),
                    Multiply => self.interpret(*lhs) * self.interpret(*rhs),
                    Divide => self.interpret(*lhs) / self.interpret(*rhs),
                    LessThan => if self.interpret(*lhs) < self.interpret(*rhs) { 1 } else { 0 },
                    LessOrEqual => if self.interpret(*lhs) <= self.interpret(*rhs) { 1 } else { 0 },
                    GreaterThan => if self.interpret(*lhs) > self.interpret(*rhs) { 1 } else { 0 },
                    GreaterOrEqual => if self.interpret(*lhs) >= self.interpret(*rhs) { 1 } else { 0 },
                    EqualEqual => if self.interpret(*lhs) == self.interpret(*rhs) { 1 } else { 0 },
                    NotEqual => if self.interpret(*lhs) != self.interpret(*rhs) { 1 } else { 0 },
                },
            Assignment { name, expr } => {
                let value = self.interpret(*expr);
                self.environment.bindings.insert(name, value);

                value
            }
            Identifier(name)  => {
                match self.environment.bindings.get(name) {
                    Some(value) => *value,
                    None => panic!("{} is undefined.", name)
                }
            }
            ValueExpression(value) => value,
            BlockExpression(expressions) => {
                for exp in expressions {
                    self.interpret(exp)
                }
            }
            WhileExpression { condition, body } => {
                while self.interpret(*condition) != 0 {
                    self.interpret(*body)
                }
            }
            IfExpression { condition, then_clause, else_clause } => {
                if self.interpret(*condition) != 0 {
                    self.interpret(*then_clause)
                } else {
                    match *else_clause {
                        Some(exp) => self.interpret(exp),
                        None => 0
                    }
                }
            }
        }
    }

    fn callMain()
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
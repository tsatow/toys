use crate::ast::{add, divide, integer, multiply, subtract, Environment, Expression, Operator, Program, TopLevel, Value};
use Operator::{Add, Divide, EqualEqual, GreaterOrEqual, GreaterThan, LessOrEqual, LessThan, Multiply, NotEqual, Subtract};
use Expression::{Assignment, BinaryExpression, BlockExpression, FunctionCall, Identifier, IfExpression, ValueExpression, WhileExpression};
use TopLevel::{FunctionDefinition, GlobalVariableDefinition};
use std::collections::HashMap;
use std::panic::panic_any;

struct Interpreter<'a> {
    environment: Environment<'a>,
    function_environment: HashMap<&'a str, &'a TopLevel<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn call_main(&mut self, program: &'a Program) -> Value {
        let topLevels = program.definitions();
        for topLevel in topLevels {
            match topLevel {
                FunctionDefinition { name, args: _, body: _ } => {
                    self.function_environment.insert(name, &topLevel);
                }
                GlobalVariableDefinition {} => todo!(),
            }
        }
        match self.function_environment.get("main") {
            Some(FunctionDefinition { name: _, args: _, body }) => self.interpret(body),
            _ => panic!("This program doesn't have main() function")
        }
    }
    fn interpret(&mut self, expression: &'a Expression<'a>) -> Value {
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
            Identifier(name) => {
                match self.environment.bindings.get(name) {
                    Some(value) => *value,
                    None => panic!("{} is undefined.", name)
                }
            }
            ValueExpression(value) => *value,
            BlockExpression { expressions } => {
                let mut ret_val = 1;
                for exp in *expressions {
                    ret_val = self.interpret(exp)
                }
                ret_val
            }
            WhileExpression { condition, body } => {
                let mut ret_val = 1;
                while self.interpret(*condition) != 0 {
                    ret_val = self.interpret(*body)
                }
                ret_val
            }
            IfExpression { condition, then_clause, else_clause } => {
                if self.interpret(*condition) != 0 {
                    self.interpret(*then_clause)
                } else {
                    match else_clause {
                        Some(exp) => self.interpret(exp),
                        None => 1
                    }
                }
            }
            FunctionCall { name, args } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10_plus_20_should_work() {
        let environment = Environment { bindings: &mut HashMap::new(), next: None };
        let function_environment: HashMap<&str, &TopLevel> = HashMap::new();
        let mut interpreter = Interpreter { environment, function_environment };
        assert_eq!(interpreter.interpret(&add(&integer(10), &integer(20))), 30)
    }
}
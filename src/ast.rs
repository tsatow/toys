use std::collections::HashMap;

pub enum TopLevel<'a> {
    GlobalVariableDefinition {},
    FunctionDefinition { name: &'a str, args: &'a [Expression<'a>], body: &'a Expression<'a> },
}

pub struct Environment<'a> {
    pub bindings: &'a mut HashMap<&'a str, Value>,
    pub next: Option<Expression<'a>>
}

pub struct Program {

}

impl Program {
    pub fn definitions<'a>(&self) -> &[TopLevel<'a>] {
        todo!()
    }
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
    EqualEqual,
    NotEqual,
}

pub enum Expression<'a> {
    BinaryExpression { operator: Operator, lhs: &'a Expression<'a>, rhs: &'a Expression<'a> },
    Assignment { name: &'a str, expr: &'a Expression<'a> },
    Identifier(&'a str),
    ValueExpression(Value),
    BlockExpression { expressions: &'a [Expression<'a>] },
    WhileExpression { condition: &'a Expression<'a>, body: &'a Expression<'a> },
    IfExpression { condition: &'a Expression<'a>, then_clause: &'a Expression<'a>, else_clause: &'a Option<Expression<'a>> },
    FunctionCall { name: &'a str, args: &'a [Expression<'a>] },
}

impl<'a> Expression<'a> {
    pub fn add(lhs: &'a Self, rhs: &'a Self) -> Self {
        Expression::BinaryExpression { operator: Operator::Add, lhs, rhs }
    }
    pub fn subtract(lhs: &'a Self, rhs: &'a Self) -> Self {
        Expression::BinaryExpression { operator: Operator::Subtract, lhs, rhs }
    }
    pub fn multiply(lhs: &'a Self, rhs: &'a Self) -> Self {
        Expression::BinaryExpression { operator: Operator::Multiply, lhs, rhs }
    }
    pub fn divide(lhs: &'a Self, rhs: &'a Self) -> Self {
        Expression::BinaryExpression { operator: Operator::Divide, lhs, rhs }
    }
    pub fn integer(int: i32) -> Self {
        Expression::ValueExpression(int)
    }
    pub fn block(expressions: &'a [Self]) -> Self {
        Expression::BlockExpression { expressions }
    }
    pub fn while_exp(condition: &'a Self, body: &'a Self) -> Self {
        Expression::WhileExpression { condition, body }
    }
    pub fn if_exp(condition: &'a Self, then_clause: &'a Self, else_clause: &'a Option<Self>) -> Self {
        Expression::IfExpression { condition, then_clause, else_clause }
    }
}

pub fn add<'a>(lhs: &'a Expression<'a>, rhs: &'a Expression<'a>) -> Expression<'a> {
    Expression::add(lhs, rhs)
}

pub fn subtract<'a>(lhs: &'a Expression<'a>, rhs: &'a Expression<'a>) -> Expression<'a> {
    Expression::subtract(lhs, rhs)
}

pub fn multiply<'a>(lhs: &'a Expression<'a>, rhs: &'a Expression<'a>) -> Expression<'a> {
    Expression::multiply(lhs, rhs)
}

pub fn divide<'a>(lhs: &'a Expression<'a>, rhs: &'a Expression<'a>) -> Expression<'a> {
    Expression::divide(lhs, rhs)
}

pub fn integer<'a>(int: i32) -> Expression<'a> {
    Expression::integer(int)
}

pub fn block<'a>(expressions: &'a [Expression<'a>]) -> Expression<'a> {
    Expression::block(expressions)
}

pub fn while_exp<'a>(condition: &'a Expression<'a>, body: &'a Expression<'a>) -> Expression<'a> {
    Expression::while_exp(condition, body)
}

pub fn if_else_exp<'a>(condition: &'a Expression<'a>, then_clause: &'a Expression<'a>, else_clause: &'a Option<Expression<'a>>) -> Expression<'a> {
    Expression::if_exp(condition, then_clause, else_clause)
}

pub type Value = i32;
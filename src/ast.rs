use std::collections::HashMap;

pub enum TopLevel {
    GlobalVariableDefinition {},
    FunctionDefinition {},
}

pub struct Environment<'a> {
    bindings: &'a mut HashMap<&'a str, Value>,
    next: Option<Expression<'a>>
}

pub struct Program {

}

impl Program {
    pub fn definitions() -> &[TopLevel] {
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
    BinaryExpression { operator: Operator, lhs: Box<Expression<'a>>, rhs: Box<Expression<'a>> },
    Assignment { name: &'a str, expr: Box<Expression<'a>> },
    Identifier(&'a str),
    ValueExpression(Value),
    BlockExpression { expressions: &'a [Expression<'a>] },
    WhileExpression { condition: Box<Expression<'a>>, body: Box<Expression<'a>> },
    IfExpression { condition: Box<Expression<'a>>, then_clause: Box<Expression<'a>>, else_clause: Option<Expression<'a>> },
    FunctionCall { name: &'a str, args: &'a [Expression<'a>], body: Box<Expression<'a>> },
}

impl Expression<'_> {
    pub fn add<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Self <'a> {
        Expression::BinaryExpression { operator: Operator::Add, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
    pub fn subtract<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
        Expression::BinaryExpression { operator: Operator::Subtract, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
    pub fn multiply<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
        Expression::BinaryExpression { operator: Operator::Multiply, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
    pub fn divide<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
        Expression::BinaryExpression { operator: Operator::Divide, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
    pub fn integer(int: i32) -> Self {
        ValueExpression(int)
    }
    pub fn block<'a>(expressions: &'a [Expression<'a>]) -> Self {
        BlockExpression { expressions }
    }
    pub fn while_exp<'a>(condition: Expression<'a>, body: Expression<'a>) -> Self {
        WhileExpression { condition: Box::new(condition), body: Box::new(body) }
    }
    pub fn if_exp<'a>(condition: Expression<'a>, then_clause: Expression<'a>, else_clause: Option<Expression<'a>>) -> Self {
        IfExpression { condition: Box::new(condition), then_clause: Box::new(body), else_clause }
    }
}

pub fn add<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::add(lhs, rhs)
}

pub fn subtract<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::subtract(lhs, rhs)
}

pub fn multiply<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::multiply(lhs, rhs)
}

pub fn divide<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
    Expression::divide(lhs, rhs)
}

pub fn integer(int: i32) -> Expression<'static> {
    Expression::integer(int)
}

pub fn block<'a>(expressions: &'a [Expression<'a>]) -> Expression<'a> {
    Expression::block(expressions)
}

pub fn while_exp<'a>(condition: Expression<'a>, body: Expression<'a>) -> Expression<'a> {
    Expression::while_exp(condition, body)
}

pub fn if_exp<'a>(condition: Expression<'a>, then_clause: Expression<'a>) -> Expression<'a> {
    Expression::if_exp(condition, then_clause, None)
}

pub fn if_else_exp<'a>(condition: Expression<'a>, then_clause: Expression<'a>, else_clause: Expression<'a>) -> Expression<'a> {
    Expression::if_exp(condition, then_clause, Some(else_clause))
}

pub type Value = i32;
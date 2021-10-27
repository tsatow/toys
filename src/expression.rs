use crate::expression::Expression::ValueExpression;

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub enum Expression<'a> {
    BinaryExpression{ operator: Operator, lhs: Box<Expression<'a>>, rhs: Box<Expression<'a>> },
    Assignment{ name: &'a str, expr: Box<Expression<'a>> },
    Identifier(&'a str),
    ValueExpression(Value),
}

impl Expression<'_> {
    pub fn add<'a>(lhs: Expression<'a>, rhs: Expression<'a>) -> Expression<'a> {
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

pub type Value = i32;
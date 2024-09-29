use crate::ast::*;

// TODO: For additional safety, we could instead have two types of visitors:
// one for mutable and one for immutable visits.
pub trait Visitor {

    // Default implementations do nothing
    fn visit_program(&mut self, _program: &mut Program) {}
    fn visit_input(&mut self, _input: &mut Input) {}
    fn visit_statement(&mut self, _statement: &mut Statement) {}
    fn visit_expression(&mut self, _expression: &mut Expression) {}
    fn visit_value(&mut self, _value: &mut Value) {}
    fn visit_operator(&mut self, _operator: &mut Operator) {}
}

pub trait NodeAccept {
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

impl NodeAccept for Program {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_program(self);
    }
}

impl NodeAccept for Input {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_input(self);
    }
}

impl NodeAccept for Statement {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_statement(self);
    }
}

impl NodeAccept for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_expression(self);
    }
}

impl NodeAccept for Value {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_value(self);
    }
}

impl NodeAccept for Operator {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_operator(self);
    }
}
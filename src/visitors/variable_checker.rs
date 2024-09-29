use std::collections::HashSet;
use crate::ast::*;
use crate::visitors::visitor::Visitor;
use crate::visitors::visitor::NodeAccept;

use crate::messages::*;

// Visitor that checks for undefined variables and redefinitions
pub struct VariableChecker {
    defined_variables: HashSet<String>,
}

impl VariableChecker {
    pub fn new() -> Self {
        VariableChecker {
            defined_variables: HashSet::new(),
        }
    }

}

impl Visitor for VariableChecker {
    fn visit_program(&mut self, program: &mut Program) {
        self.defined_variables.clear();

        // Arguments are considered variables
        for input in &mut program.inputs {
            input.accept(self);
        }

        for statement in &mut program.statements {
            statement.accept(self);
        }
    }

    fn visit_input(&mut self, input: &mut Input) {
        if !self.defined_variables.insert(input.name.clone()) {
            error(&format!("Redefinition of input variable '{}'.", input.name));
        }
    }

    fn visit_statement(&mut self, statement: &mut Statement) {
        match statement {
            Statement::Assign { variable, expression } => {
                if !self.defined_variables.insert(variable.clone()) {
                    error(&format!("Redefinition of variable '{}'.", variable));
                }

                expression.accept(self);
            }
        }
    }

    fn visit_expression(&mut self, expression: &mut Expression) {
        match expression {
            Expression::Binary { left, right, .. } => {
                left.accept(self);
                right.accept(self);
            }
            Expression::Value(value) => {
                value.accept(self);
            }
        }
    }

    fn visit_value(&mut self, value: &mut Value) {
        match value {
            Value::Identifier(name) => {
                // Check if the variable is defined
                if !self.defined_variables.contains(name) {
                    error(&format!("Use of undefined variable '{}'.", name));
                }
            }
            Value::Expression(expr) => {
                // Visit nested expressions
                expr.accept(self);
            }
            Value::Integer(_) => {
                // Do nothing for integer literals
            }
        }
    }
}
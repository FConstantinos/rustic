use std::collections::HashMap;
use crate::ast::*;
use crate::visitors::visitor::Visitor;
use crate::visitors::visitor::NodeAccept;
use crate::messages::*;

// Visitor that performs constant propagation
// 	- If an expression is encountered: 
// 		- if it's a Value::Integer, do nothing
// 		- if it's a Value::Expression that contains an integer, replace it with a Value::Integer
// 		- if it's a Value::Identifier:
// 			- Check whether that identifier is cached as a constant
// 				- if yes, replace the node with Value::Integer where the value of the integer is the value of the constant
// 		- if it's a Binary expression:
// 			- recursively traverse the LHS and RHS
// 			- if both LHS and RHS reduce to Value::Integer, apply the binary operation to the integers
// 				- With the appropriate checks for a correct result
// 			
//     - if a Statment node is encountered (i.e an assignment):
// 		- recursively traverse the initializing expression.
// 		- If that expression is evaluated to a constant after traversal, cache the name of the constant and its value
pub struct ConstantPropagation {
    // A map to store the current known constants for variables
    constants: HashMap<String, u8>,
}

impl ConstantPropagation {
    pub fn new() -> Self {
        ConstantPropagation {
            constants: HashMap::new(),
        }
    }
}

impl Visitor for ConstantPropagation {
    fn visit_program(&mut self, program: &mut Program) {
        // Clear the constants map for a new run
        self.constants.clear();

        // Visit each statement in the program.
        // There is no need to visit inputs as they
        // are considered non-constant variables
        for statement in &mut program.statements {
            statement.accept(self);
        }
    }

    fn visit_statement(&mut self, statement: &mut Statement) {
        match statement {
            Statement::Assign { variable, expression } => {
                // Visit the initializing expression to propagate constants
                expression.accept(self);

                // If the expression is a constant value, store it in the map
                match expression {
                    Expression::Value(boxed_value) => {
                        if let Value::Integer(val) = **boxed_value {
                            self.constants.insert(variable.clone(), val);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn visit_expression(&mut self, expression: &mut Expression) {
        match expression {
            Expression::Binary { left, operator, right } => {
                // Visit left and right expressions to propagate constants
                left.accept(self);
                right.accept(self);

                // Simplify if possible
                match (left, &**right) {
                    (Value::Integer(left_val), Expression::Value(boxed_value)) => {
                        match &**boxed_value {
                            Value::Integer(right_val) => {
                                let result = match operator {
                                    Operator::Add => {
                                        let res = (*left_val as u16) + (*right_val as u16);
                                        if res > u8::MAX as u16{
                                            error(&format!("Constant evaluation resulted in value greater than 255: {} + {}", left_val, right_val));
                                        } else {
                                            res as u8
                                        }
                                    }
                                    Operator::Subtract => {
                                        if *left_val < *right_val {
                                            error(&format!("Constant evaluation resulted in negative value: {} - {}", left_val, right_val));
                                        } else {
                                            *left_val - *right_val
                                        }
                                    }
                                    Operator::Multiply => {
                                        let res: u16 = (*left_val as u16) * (*right_val as u16);
                                        if res > u8::MAX as u16{
                                            error(&format!("Constant evaluation resulted in value greater than 255: {} * {}", left_val, right_val));
                                        } else {
                                            res as u8
                                        }    
                                    }
                                    Operator::Divide => {
                                        if *right_val == 0 {
                                            error(&format!("Constant evaluation resulted in division by zero: {} / {}", left_val, right_val));
                                        } else if (*left_val % *right_val) != 0 {
                                            error(&format!("Constant evaluation resulted in non-integer division: {} / {}", left_val, right_val));
                                        } else {
                                            *left_val / *right_val
                                        }
                                    }
                                };
                                *expression = Expression::Value(Box::new(Value::Integer(result)));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Expression::Value(value) => {
                value.accept(self);
            }
        }
    }

    fn visit_value(&mut self, value: &mut Value) {
        match value {

            // If the value is an identifier, check if it is a constant
            Value::Identifier(ref var) => {
                if (self.constants).contains_key(var) {

                    // Replace the identifier with its constant value
                    *value = Value::Integer(self.constants[var]);
                }
            }

            // If the expression is a constant, replace it with the constant value
            Value::Expression(expr) => {
                expr.accept(self);
                match &**expr {
                    Expression::Value(boxed_value) => {
                        if let Value::Integer(val) = **boxed_value {
                            *value = Value::Integer(val);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

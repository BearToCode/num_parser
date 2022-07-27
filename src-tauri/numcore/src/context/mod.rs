use std::collections::HashMap;

use crate::objects::Expression;

/// Contains user-defined functions and constants.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Context {
    pub functions: HashMap<String, (Vec<String>, Box<Expression>)>,
    pub variables: HashMap<String, Box<Expression>>,
}

impl Context {
    /// Generates an empty context.
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn join_with(&mut self, context: &Self) {
        for (identifier, (params, body)) in context.functions.clone() {
            self.add_function(identifier, params, body);
        }
        for (identifier, expression) in context.variables.clone() {
            self.add_variable(identifier, expression)
        }
    }

    /// Add a function to the user-defined ones.
    pub fn add_function(&mut self, identifier: String, params: Vec<String>, body: Box<Expression>) {
        self.functions.insert(identifier, (params, body));
    }

    // Add a variable to the user-defined ones.
    pub fn add_variable(&mut self, identifier: String, expression: Box<Expression>) {
        self.variables.insert(identifier, expression);
    }

    /// Returns a user-defined function given an identifier.
    pub fn get_function(&self, identifier: &str) -> Option<(Vec<String>, Box<Expression>)> {
        self.functions.get(identifier).cloned()
    }

    /// Returns a user-defined constant given an identifier.
    pub fn get_var(&self, identifier: &str) -> Option<Box<Expression>> {
        self.variables.get(identifier).cloned()
    }

    /// Returns true if the identifier refers to a user-defined function.
    pub fn is_function(&self, identifier: &str) -> bool {
        if let Some(_) = self.get_function(identifier) {
            true
        } else {
            false
        }
    }

    /// Returns true if the identifier refers to a user-defined constant.
    pub fn is_var(&self, identifier: &str) -> bool {
        if let Some(_) = self.get_var(identifier) {
            true
        } else {
            false
        }
    }
}

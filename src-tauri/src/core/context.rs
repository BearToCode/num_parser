use super::functions::Expression;
use std::collections::HashMap;

pub struct Context<'a> {
    pub definitions: HashMap<&'a str, f64>, // Ex: x = 4
    pub functions: Vec<&'a Expression<'a>>, // All custom functions provided by the user
}

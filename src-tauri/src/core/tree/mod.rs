use super::function::*;

/// A node in the expression tree.
pub struct Node {
    children: Vec<Node>,
    function: Function,
}

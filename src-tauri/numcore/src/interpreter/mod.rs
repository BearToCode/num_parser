use crate::{
    objects::Expression,
    objects::Request,
    out::{ErrorType, EvalResult},
    token::{self, tokentype::TokenType},
    tree::{Node, Tree},
};

impl Node {
    fn branches(&self) -> Vec<&Node> {
        match self {
            Self::Binary(lhs, _, rhs) => vec![&lhs, &rhs],
            Self::Unary(_, node) => vec![&node],
            Self::Func(_, node) => vec![&node],
            Self::Literal(_) | Self::Var(_) => vec![],
        }
    }
}

/// Converts a tree into a comprehensible request by the user.
pub fn interpret_tree(tree: &Tree) -> EvalResult<Request> {
    let equal_tokens = match_all(&tree.0, 0, &|node| match node {
        Expression::Binary(_, token_type, _) => *token_type == TokenType::Equal,
        _ => false,
    });

    if let Some(equals) = equal_tokens {
        // Function or var declaration
        if equals.len() > 1 {
            return Err(ErrorType::InvalidTokenPosition {
                token: TokenType::Equal,
            });
        }

        let equal_node_info = &equals[0];
        // Avoid equals in brackets
        if equal_node_info.depth != 0 {
            return Err(ErrorType::InvalidTokenPosition {
                token: TokenType::Equal,
            });
        }

        let (left, right) = match equal_node_info.node {
            Expression::Binary(left_expr, operator, right_expr) => {
                if *operator != TokenType::Equal {
                    return Err(ErrorType::InternalError {
                        message: "operator was not of expected type".to_owned(),
                    });
                } else {
                    (&(**left_expr), &(**right_expr))
                }
            }
            _ => {
                return Err(ErrorType::InternalError {
                    message: "node was not of expected type".to_owned(),
                })
            }
        };

        match left {
            Expression::Var(identifier) => {
                return Ok(Request::VarDeclaration(
                    identifier.clone(),
                    Box::new(right.clone()),
                ))
            }
            Expression::Func(identifier, arguments_node) => {
                // Retrieve all sub nodes
                let sub_nodes = match_all(&arguments_node, 0, &|_| true);
                // Retrieve function parameters
                match sub_nodes {
                    None => {
                        return Ok(Request::FuncDeclaration(
                            identifier.clone(),
                            vec![],
                            Box::new(right.clone()),
                        ))
                    }
                    Some(nodes) => {
                        let mut params = vec![];
                        for node_info in nodes {
                            match node_info.node {
                                Expression::Binary(_, op, _) => {
                                    // Exclude all the aggregator operators
                                    if *op != TokenType::Comma {
                                        return Err(ErrorType::InvalidDeclaration);
                                    }
                                }
                                Expression::Var(arg) => params.push(arg.clone()),
                                _ => return Err(ErrorType::InvalidDeclaration),
                            }
                        }
                        return Ok(Request::FuncDeclaration(
                            identifier.clone(),
                            params,
                            Box::new(right.clone()),
                        ));
                    }
                }
            }
            _ => return Err(ErrorType::InvalidDeclaration),
        }
    } else {
        return Ok(Request::Evaluation(Box::new(tree.0.clone())));
    }
}

#[derive(Clone)]
struct NodeInfo<'a> {
    node: &'a Node,
    depth: u32,
}

/// Finds all matches within a tree.
fn match_all<'a, P>(
    start: &'a Node,
    starting_depth: u32,
    predicate: &P,
) -> Option<Vec<NodeInfo<'a>>>
where
    P: Fn(&Node) -> bool,
{
    if predicate(start) {
        Some(vec![NodeInfo {
            node: start,
            depth: starting_depth,
        }])
    } else {
        let mut out = vec![];
        for branch in start.branches() {
            if let Some(results) = match_all(branch, starting_depth + 1, predicate) {
                out = [out, results].concat();
            }
        }
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }
}

use crate::{
    objects::Expression,
    objects::Request,
    out::{ErrorType, EvalResult},
    token::tokentype::TokenType,
    tree::{Node, Tree},
};

impl Node {
    fn branches(&self) -> Vec<&Node> {
        match self {
            Self::Binary(lhs, _, rhs) => vec![&lhs, &rhs],
            Self::Unary(_, node) => vec![&node],
            Self::Func(_, nodes) => {
                (*nodes.iter().map(|x| &(**x)).collect::<Vec<&Node>>()).to_vec()
            }
            Self::Literal(_) | Self::Var(_) => vec![],
            Self::Union(nodes) => (*nodes.iter().map(|x| &(**x)).collect::<Vec<&Node>>()).to_vec(),
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
                let mut params: Vec<String> = vec![];
                for node_box in arguments_node {
                    let node = &**node_box;
                    match node {
                        Expression::Var(arg_name) => params.push(arg_name.clone()),
                        _ => return Err(ErrorType::InvalidDeclaration),
                    }
                }
                return Ok(Request::FuncDeclaration(
                    identifier.clone(),
                    params,
                    Box::new(right.clone()),
                ));
            }
            _ => return Err(ErrorType::InvalidDeclaration),
        }
    } else {
        return Ok(Request::Evaluation(Box::new(tree.0.clone())));
    }
}

#[derive(Clone, Debug)]
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
    let mut out = vec![];

    if predicate(start) {
        out.push(NodeInfo {
            node: start,
            depth: starting_depth,
        });
    }
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

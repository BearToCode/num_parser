mod checks;
mod op_conversion;
mod utils;

use super::functions::operators::*;
use super::functions::{Expression, Function};

use self::utils::{get_function_name, is_const_or_variable};

pub fn parse_expression<'a>(expression: &'a str) -> Result<Expression, String> {
    let mut string = String::from(expression);

    if string.len() == 0 {
        return Err(String::from("No expression provided!"));
    }

    // Remove spaces
    string = string.replace(" ", "");

    // Checks
    checks::are_brackets_valid(&string)?;

    checks::are_operators_valid(&string)?;

    // Convert operators
    op_conversion::operators_to_functions(&mut string)?;

    // Build the expression
    return build_expression(&string);
}

// Input shall be of type function(arg1, arg2, ...)
fn build_expression(expression: &String) -> Result<Expression, String> {
    match is_const_or_variable(expression)? {
        Some(exp) => return Ok(exp),
        None => (),
    };

    let content: Vec<String> = get_function_content(expression)?;

    // Get the corresponding function
    match get_function_name(expression) {
        Err(err) => Err(err),
        Ok(name) => match &name[..] {
            "sum" => return Ok(Sum::build(get_function_parameters(&content)?)?),
            "sub" => return Ok(Subtraction::build(get_function_parameters(&content)?)?),
            "mul" => return Ok(Multiplication::build(get_function_parameters(&content)?)?),
            "div" => return Ok(Division::build(get_function_parameters(&content)?)?),
            other => return Err(format!("Invalid function name: {}!", other)),
        },
    }
}

fn get_function_content(function: &String) -> Result<Vec<String>, String> {
    let mut mut_function = function.clone();

    // Remove closing bracket
    if mut_function.chars().nth_back(0).unwrap() != ')' {
        return Err(String::from(
            "Error while retrieving function content: last character is not a closing bracket!",
        ));
    } else {
        mut_function.replace_range(mut_function.len() - 1..mut_function.len(), "");
    }

    while mut_function.len() > 1 {
        let char = mut_function.chars().nth(0).unwrap();

        if char.is_alphabetic() {
            // Remove char
            mut_function.replace_range(0..1, "");
        } else if char == '(' {
            mut_function.replace_range(0..1, "");

            // Split and return the string
            let mut split_content: Vec<String> = vec![];

            let mut depth = 0;
            let mut last_cut = 0;

            for (index, char) in mut_function.chars().enumerate() {
                if char == '(' {
                    depth += 1;
                } else if char == ')' {
                    depth -= 1;
                } else if char == ',' {
                    if depth == 0 {
                        split_content.push(String::from(&mut_function[last_cut..index]));
                        last_cut = index + 1;
                    }
                }
            }

            // Add final sequence
            split_content.push(String::from(&mut_function[last_cut..mut_function.len()]));

            return Ok(split_content);
        } else {
            return Err(format!(
                "Error while retrieving content: invalid char '{}' !",
                char
            ));
        }
    }

    Err(String::from(
        "Error while retrieving function content: could not find a function!",
    ))
}

fn get_function_parameters(content: &Vec<String>) -> Result<Vec<Expression>, String> {
    let mut out_vec: Vec<Expression> = vec![];

    for expression in content {
        out_vec.push(build_expression(expression)?);
    }

    Ok(out_vec)
}

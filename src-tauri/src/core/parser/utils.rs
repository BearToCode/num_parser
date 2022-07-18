use super::Expression;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

pub fn get_adjacent_expression_end_index(
    string: &String,
    start: usize,
    dir: Direction,
) -> Result<usize, &'static str> {
    // R: When going right we end with:
    // 1. a bracket,
    // 2. a number, and then we check for:
    //    - bracket,
    // 	  - function,
    // 	  - operator
    // L: When going left:
    // 1. a spacing bracket, which occurs when there before the bracket there is not a function(letters) but an operator
    // 2. a function
    // 3. a number

    let mut index = start;
    let mut numbers_found = false;
    let mut relative_depth = 0;

    // Only needed when going left
    let mut text_found = false;
    let mut expect_function = false;

    loop {
        let current_char = string.chars().nth(index).unwrap();

        if dir == Direction::Left && text_found && !current_char.is_alphabetic() {
            // L:2
            return Ok(index + 1);
        } else if !current_char.is_digit(10) && numbers_found {
            if dir == Direction::Right {
                // R:2
                return Ok(index);
            } else {
                // L:3
                return Ok(index + 1);
            }
        }

        if current_char == '(' {
            relative_depth += 1;
            if dir == Direction::Left && relative_depth == 0 {
                expect_function = true;
            }
        } else if current_char == ')' {
            relative_depth -= 1;
            if dir == Direction::Right && relative_depth == 0 {
                // R:1
                return Ok(index);
            }
        } else if current_char.is_digit(10) && relative_depth == 0 {
            numbers_found = true;
        } else {
            if dir == Direction::Left && relative_depth == 0 {
                if expect_function {
                    if current_char.is_alphabetic() {
                        // Function
                        text_found = true;
                        expect_function = false;
                    } else {
                        // Operator, case L:1
                        return Ok(index + 1);
                    }
                }
            }

            if dir == Direction::Right && numbers_found && relative_depth == 0 {
                // R:2 -> operators and letters (function)
                return Ok(index - 1);
            }
        }

        if !(index > 0 && index < string.len() - 1) {
            break;
        };

        index = match dir {
            Direction::Left => index - 1,
            Direction::Right => index + 1,
        };
    }
    if numbers_found || dir == Direction::Left && (text_found || expect_function) {
        return if dir == Direction::Left {
            Ok(0)
        } else {
            Ok(string.len())
        };
    } else {
        Err("Error while parsing operators!")
    }
}

pub fn is_valid_number(string: &String, radix: u32) -> bool {
    let mut comma_found = false;
    for char in string.chars() {
        if !(char.is_digit(radix) || char == '.' && !comma_found) {
            return false;
        } else if char == '.' {
            comma_found = true;
        }
    }
    true
}

pub fn is_const_or_variable(expression: &String) -> Result<Option<Expression>, String> {
    if expression.contains('(') {
        return Ok(None);
    }

    // const
    if is_valid_number(&expression, 10) {
        return Ok(Some(Expression::Const {
            value: expression.parse::<f64>().unwrap(),
        }));
    // var
    } else if expression.len() == 1 {
        return Ok(Some(Expression::Variable {
            identifier: expression.chars().nth(0).unwrap(),
        }));
    } else {
        return Err(format!(
            "Expression: '{}' is not a function, but neither a number or a variable!",
            expression
        ));
    }
}

pub fn get_function_name(expression: &String) -> Result<String, String> {
    let mut index = 0;
    let mut name: String = String::from("");
    while index < expression.len() {
        let char = expression.chars().nth(index).unwrap();
        if char.is_alphabetic() {
            name.push(char);
        } else if char == '(' {
            return Ok(name);
        } else {
            return Err(format!(
                "Error while parsing expression: invalid character '{}' !",
                char
            ));
        }
        index += 1;
    }

    Err(String::from(
        "Error while parsing expression: could not find a function!",
    ))
}

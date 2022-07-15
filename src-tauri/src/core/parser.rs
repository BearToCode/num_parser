use super::functions::Expression;

pub fn parse_function<'a>(s: &'a str) -> Result<Expression, String> {
    let mut string = String::from(s);

    // Check brackets
    match are_brackets_valid(&string) {
        Ok(()) => (),
        Err(error) => return Err(String::from(error)),
    }

    // Remove spaces
    string = string.replace(" ", "");

    println!("{}", string);

    // Convert operators
    match operators_to_functions(&mut string) {
        Ok(()) => (),
        Err(err) => return Err(String::from(err)),
    };

    Err(String::from(string))
}

fn are_brackets_valid(string: &String) -> Result<(), String> {
    let mut current_depth: i16 = 0;
    for (i, char) in string.chars().enumerate() {
        if char == '(' {
            current_depth += 1;
        } else if char == ')' {
            current_depth -= 1;
            if current_depth < 0 {
                let message = format!("Invalid bracket at position: {}", i);
                return Err(message);
            }
        }
    }

    if current_depth != 0 {
        return Err(String::from("Invalid brackets!"));
    } else {
        Ok(())
    }
}

fn operators_to_functions(string: &mut String) -> Result<(), &'static str> {
    loop {
        let (current_operator_index, current_operator_name) =
            match get_highest_priority_operator(&string) {
                Some(result) => result,
                None => break,
            };

        // Replace operator char with a comma
        string.replace_range(current_operator_index..current_operator_index + 1, ",");

        println!("comma: {}", string);

        let left_expression_end = match get_adjacent_expression_end_index(
            &string,
            current_operator_index,
            Direction::Left,
        ) {
            Ok(result) => result,
            Err(msg) => return Err(msg),
        };

        println!("right...");

        let right_expression_end = match get_adjacent_expression_end_index(
            string,
            current_operator_index,
            Direction::Right,
        ) {
            Ok(result) => result,
            Err(msg) => return Err(msg),
        };

        // Place closing bracket at the end of the right expression
        string.insert(right_expression_end, ')');
        // Place function and opening bracket at the beginning of the left expression
        string.insert_str(
            left_expression_end,
            &(current_operator_name.to_owned() + "("),
        );
    }
    println!("{}", string);
    Ok(())
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

fn get_adjacent_expression_end_index(
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

        println!("{}", current_char);

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
            println!("index: {}, len: {}", index, string.len());
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

// Returns the index and the function of the highest priority operator
fn get_highest_priority_operator(string: &String) -> Option<(usize, &'static str)> {
    let mut current_depth: u16 = 0;
    let mut highest_priority_operator_index: usize = 0;
    let mut highest_priority_operator_name: &str = "";
    let mut highest_priority: (u16, u8) = (0, 0);
    for (i, char) in string.chars().enumerate() {
        if char == '(' {
            current_depth += 1;
        } else if char == ')' {
            current_depth -= 1;
        }

        let (operator_name, operator_priority) =
            match super::functions::operators::operator_data_from_char(&char) {
                Some(value) => value,
                None => continue,
            };

        let this_priority = (current_depth, operator_priority);

        if this_priority > highest_priority {
            highest_priority = this_priority;
            highest_priority_operator_name = operator_name;
            highest_priority_operator_index = i;
            println!("high: {}", highest_priority_operator_index);
        }
    }

    println!("high: {}", highest_priority_operator_index);

    println!("{:?}", highest_priority);

    if highest_priority == (0, 0) {
        None
    } else {
        Some((
            highest_priority_operator_index,
            highest_priority_operator_name,
        ))
    }
}

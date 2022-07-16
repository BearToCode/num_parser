use crate::core::functions::operators::operator_data_from_char;

pub fn are_brackets_valid(string: &String) -> Result<(), String> {
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

pub fn are_operators_valid(string: &String) -> Result<(), String> {
    // Operators are non digit, non alphabetic characters and non brackets
    for (index, char) in string.chars().enumerate() {
        if !is_possible_operator(&char) {
            continue;
        }
        match operator_data_from_char(&char) {
            Some(_) => (),
            None => {
                return Err(String::from(format!(
                    "Invalid operator '{}' at position: {}",
                    char, index
                )))
            }
        }
    }
    Ok(())
}

fn is_possible_operator(char: &char) -> bool {
    !char.is_alphanumeric() && *char != '(' && *char != ')'
}

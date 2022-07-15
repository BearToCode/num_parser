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

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

use std::io;

fn main() {
    println!("Welcome to Rust calculator!\n");
    println!("Enter the mathematical expression:");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read line");
    if !is_valid_expression(&expression) {
        println!("Invalid expression!");
        return
    }
    println!("Result: {}", expression);
}

/// Check if a given expression contains only valid characters (digits, operators, and parentheses).
fn is_valid_expression(expression: &str) -> bool {
    // Define a regular expression pattern to match valid characters in the expression.
    let pattern = regex::Regex::new(r"^[0-9()+\-*/\s]*$").unwrap();

    // Use the regex pattern to check if the expression contains only valid characters.
    pattern.is_match(expression)
}

/// Evaluate a mathematical expression in postfix notation (RPN - Reverse Polish Notation).
fn evaluate_expression(expression: &str) -> f64 {
    let mut values = Vec::new(); // Stack to store numbers
    //let mut operators = Vec::new(); // Stack to store operators

    let mut i = 0; // Index of the current character in the expression
    while i < expression.len() {
        match expression.chars().nth(i).unwrap() {
            '0'..='9' => {
                // Parse and push numbers onto the values stack
                let mut j = i;
                while j < expression.len() && expression.chars().nth(j).unwrap().is_digit(10) {
                    j+=1;
                }
                let number: f64 = expression[i..j].parse().expect("Invalid number");
                values.push(number);
                i = j;
            }
            _ => {
                panic!("Invalid character in expression.");
            }
        }
    }
    
    values.pop().expect("Invalid expression") // The final number is the result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_expression() {
        // Test valid expressions
        assert!(is_valid_expression("1+2"));
        assert!(is_valid_expression("1+2*3"));
        assert!(is_valid_expression("1+2*3/4"));
        assert!(is_valid_expression("1+2*3/4- 5"));
        assert!(is_valid_expression("(1+2)*3/4"));
        // Test invalid expressions
        assert!(!is_valid_expression("1+2a3"));
        assert!(!is_valid_expression("1+2*3e"));
    }
}
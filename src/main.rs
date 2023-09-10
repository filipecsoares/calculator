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

/// Evaluate a mathematical expression in postfix notation (RPN).
fn evaluate_expression(expression: &str) -> f64 {
    0.0
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
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
    let expression = evaluate_expression(&expression);
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
    let mut operators = Vec::new(); // Stack to store operators

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
            '+' | '-' | '*' | '/' => {
                // Handle operators based on precedence and associativity
                while let Some(op) = operators.last() {
                    if *op != '(' && precedence(*op) >= precedence(expression.chars().nth(i).unwrap()) {
                        apply_operator(&mut values, operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(expression.chars().nth(i).unwrap());
                i += 1;
            }
            '(' => {
                // Push opening parentheses onto the operators stack
                operators.push('(');
                i += 1;
            }
            ')' => {
                // Handle closing parentheses and apply operators
                while let Some(op) = operators.last() {
                    if *op == '(' {
                        break;
                    }
                    apply_operator(&mut values, operators.pop().unwrap());
                }
                operators.pop(); // Pop the '('
                i += 1;
            }
            ' ' => {
                i += 1; // Skip whitespace
            }
            _ => {
                panic!("Invalid character in expression.");
            }
        }
    }

    // Apply any remaining operators
    while let Some(op) = operators.pop() {
        apply_operator(&mut values, op);
    }
    
    values.pop().expect("Invalid expression") // The final number is the result
}

/// Define operator precedence for +, -, *, and / operators.
fn precedence(operator: char) -> usize {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0, // Default precedence for other characters (e.g., parentheses)
    }
}

/// Apply an operator to the top two values on the stack and push the result back onto the stack.
fn apply_operator(values: &mut Vec<f64>, operator: char) {
    let b = values.pop().expect("Invalid expression");
    let a = values.pop().expect("Invalid expression");
    let result = match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => panic!("Invalid operator"),
    };
    values.push(result);
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

    #[test]
    fn test_evaluate_expression() {
        assert_eq!(evaluate_expression("1+2"), 3.);
        assert_eq!(evaluate_expression("1+2*3"), 7.);
        assert_eq!(evaluate_expression("1-1"), 0.);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(precedence('+'), 1);
        assert_eq!(precedence('-'), 1);
        assert_eq!(precedence('*'), 2);
        assert_eq!(precedence('/'), 2);
        assert_eq!(precedence('('), 0);
    }

    #[test]
    fn test_apply_operator() {
        let mut values = Vec::new();
        values.push(1.);
        values.push(2.);
        apply_operator(&mut values, '+');
        assert_eq!(values, [3.]);
    }

    #[test]
    fn test_evaluate_expression_with_parentheses() {
        assert_eq!(evaluate_expression("(1+2)*3"), 9.);
        assert_eq!(evaluate_expression("1+(2*3)"), 7.);
        assert_eq!(evaluate_expression("1+2*(2+2)"), 9.);
        assert_eq!(evaluate_expression("(1+2)/2"), 1.5);
    }

    #[test]
    fn test_evaluate_expression_with_spaces() {
        assert_eq!(evaluate_expression("1 + 2"), 3.);
        assert_eq!(evaluate_expression("1 + 2 * 3"), 7.);
        assert_eq!(evaluate_expression("1 - 1"), 0.);
    }
}
# Rust Calculator

Calculator project developed to practice Rust and Math.

## How It Works

### Input Validation

The calculator first checks if the entered expression contains only valid characters. Valid characters include digits, basic arithmetic operators (+, -, *, /, ^), and parentheses. Any other characters are considered invalid.

### Shunting Yard Algorithm

The Shunting Yard algorithm is used to convert the infix mathematical expression (e.g., "2+2*4") into postfix notation (e.g., "224*+"). This conversion ensures that operators with higher precedence are placed earlier in the postfix expression, allowing for correct evaluation.

- [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm);

### Expression Evaluation

Once the expression is in postfix notation, the calculator evaluates it using a stack-based approach. It iterates through the postfix expression, processing numbers and operators according to their precedence and associativity rules. The result is computed and pushed onto the stack until the entire expression is evaluated.

## Usage

```sh
git clone git@github.com:filipecsoares/calculator.git

cd calculator

cargo run
```

## License

MIT License

use std::io::{self, stdout, Write};
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
    Plus,
    Minus,
    Star,
    ForwardSlash,
    Number,
}

struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    fn new(literal: String, token_type: TokenType) -> Self {
        Self{
            token_type,
            literal
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:?}: {}>", self.token_type, self.literal)
    }
}

fn main() {
    println!("Welcome to TC - the Terminal Calculator!");

    loop {
        print!("TC> ");

        stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);

                if input.trim() == "quit" || input.trim() == "q" {
                    println!("quitting...");
                    break;
                }

                let tokens = tokenize(&input);

                println!("Tokens:");
                for token in tokens {
                    println!("{}", token);
                }
            }
            Err(error) => println!("error: {}", error),
        }

    }
}

fn make_number(num_str: String) -> Token {
    Token::new(num_str, TokenType::Number)
}

fn tokenize(source: &str) -> Vec<Token> {
    let source_chars: Vec<_> = source.chars().collect();

    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;

    while i < source_chars.len() - 1 {
        let c = source_chars[i];
        i += 1;
        match c {
            ' ' => continue,
            'A'..='Z' | 'a'..='z' => {
                println!("Got Char: {}", c);
                continue;
            },
            '0'..='9' => {
                println!("Got Digit: {}", c);

                let mut num_str: String = String::from(c);

                let mut has_period = false;

                while i < source_chars.len() - 1 && (source_chars[i].is_digit(10) || source_chars[i] == '.') {
                    if source_chars[i] == '.' {
                        if has_period == false {
                            has_period = true;
                        } else {
                            // TODO: Multiple periods, what to do - what to do....?
                            // That's an error...
                            // Or should the default be to ignore it..?
                            i += 1;
                            continue;
                        }
                    } 

                    num_str.push_str(&String::from(source_chars[i]));
                    i += 1;
                }

                println!("Got: {}", num_str);
                tokens.push(Token::new(num_str, TokenType::Number));
            },
            '+' => {
                println!("Found PLUS");
                tokens.push(Token::new("+".to_string(), TokenType::Plus));
            },
            '-' => {
                println!("Found MINUS");
                tokens.push(Token::new("-".to_string(), TokenType::Minus));
            },
            '*' => {
                println!("Found STAR");
                tokens.push(Token::new("*".to_string(), TokenType::Star));
            },
            '/' => {
                println!("Found FORWARD_SLASH");
                tokens.push(Token::new("/".to_string(), TokenType::ForwardSlash));
            },
            _ => continue,
        }
    }


    return tokens;
}

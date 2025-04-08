// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

fn main() {
    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }

        Ok(code) => code,
    };

    let tokens = match lex(&code) {
        Err(error_message) => {
            println!("**Error**");
            println!("----------------------");
            println!("{}", error_message);
            println!("----------------------");
            return;
        }

        Ok(data) => data,
    };

    // print out the lexer tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("File Contents:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
        println!("{:?}", t);
    }
}

// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it,
// but Plus, Subtract, Multiply, etc. have no values associated with it.
#[derive(Debug, PartialEq)] //clone
enum Token {
    Plus,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Assign,
    Num(i32),
    Ident(String),
    If,
    While,
    Read,
    Func,
    Return,
    Int,
    Break,
    Print,
    Else,
    Continue,
    End,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equality,
    NotEqual,
}

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, String>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message represented as a string
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }

// This is a lexer that parses numbers and math operations
fn lex(code: &str) -> Result<Vec<Token>, String> {
    let bytes = code.as_bytes();
    let mut tokens: Vec<Token> = vec![];

    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }

            '-' => {
                tokens.push(Token::Subtract);
                i += 1;
            }

            '*' => {
                tokens.push(Token::Multiply);
                i += 1;
            }

            '/' => {
                tokens.push(Token::Divide);
                i += 1;
            }

            '%' => {
                tokens.push(Token::Modulus);
                i += 1;
            }

            '=' => {
                //assign
                i += 1;
                if i < bytes.len() {
                    if bytes[i] as char == '=' {
                        tokens.push(Token::Equality);
                        i += 1;
                    } else {
                        tokens.push(Token::Assign);
                    }
                } else {
                    tokens.push(Token::Assign);
                }
            }

            '0'..='9' => {
                //Num
                let start = i;
                i += 1;
                while i < bytes.len() {
                    let digit = bytes[i] as char;
                    if digit >= '0' && digit <= '9' {
                        i += 1;
                    } else {
                        break;
                    }
                }
                let end = i;
                let string_token = &code[start..end];
                let number_value = string_token.parse::<i32>().unwrap();
                let token = Token::Num(number_value);
                tokens.push(token);
            }

            'a'..='z' | 'A'..='Z' => {
                //Identifiers and keywords
                let start = i;
                i += 1;
                while i < bytes.len() {
                    let letter = bytes[i] as char;
                    if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z') {
                        i += 1;
                    } else {
                        break;
                    }
                }
                let end = i;
                let word = &code[start..end];
                match word {
                    //tests for keywords
                    "func" => tokens.push(Token::Func),
                    "return" => tokens.push(Token::Return),
                    "int" => tokens.push(Token::Int),
                    "print" => tokens.push(Token::Print),
                    "read" => tokens.push(Token::Read),
                    "while" => tokens.push(Token::While),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "break" => tokens.push(Token::Break),
                    "continue" => tokens.push(Token::Continue),

                    //word is a variable name
                    _ => tokens.push(Token::Ident(String::from(word))),
                }
            }

            //spaces | newlines -> ignore
            ' ' | '\n' => i += 1,

            '#' => {
                //comments
                // let start = i;
                while i < bytes.len() {
                    let comment_char = bytes[i] as char;
                    if comment_char == '\n' {
                        break;
                    } else {
                        i += 1;
                    }
                }
                // let end = i;
                // let comment_token = &code[start..end];
                // println!("This is a comment: {comment_token}");
                i += 1;
            }

            '(' => {
                tokens.push(Token::LeftParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                i += 1;
            }
            '{' => {
                tokens.push(Token::LeftCurly);
                i += 1;
            }
            '}' => {
                tokens.push(Token::RightCurly);
                i += 1;
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                i += 1;
            }
            ']' => {
                tokens.push(Token::RightBracket);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                i += 1;
            }
            '<' => {
                i += 1;
                if i < bytes.len() {
                    if bytes[i] as char == '=' {
                        tokens.push(Token::LessEqual);
                        i += 1;
                    } else {
                        tokens.push(Token::Less);
                    }
                } else {
                    tokens.push(Token::Less);
                }
            }

            '>' => {
                i += 1;
                if i < bytes.len() {
                    if bytes[i] as char == '=' {
                        tokens.push(Token::GreaterEqual);
                        i += 1;
                    } else {
                        tokens.push(Token::Greater);
                    }
                } else {
                    tokens.push(Token::Greater);
                }
            }

            '!' => {
                i += 1;
                if i < bytes.len() {
                    if bytes[i] as char == '=' {
                        tokens.push(Token::NotEqual);
                        i += 1;
                    } else {
                        return Err(format!("Unrecognized symbol: '{}'", c));
                    }
                } else {
                    return Err(format!("Unrecognized symbol: '{}'", c));
                }
            }

            _ => {
                //Error case
                return Err(format!("Unrecognized symbol: '{}'", c));
            }
        }
    }

    tokens.push(Token::End);
    return Ok(tokens);
}

// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;
    use Token::*;

    macro_rules! test_lex {//template for testing
        ($input:expr, [$( $token:expr ),* $(,)?]) => {
            let tokens = lex($input).unwrap();
            let expected = vec![$($token),*, End];
            assert_eq!(tokens, expected, "Failed on input: {}", $input);
        }
    }

    #[test]
    fn invalid_char_tests() {
        //NOT A COMPREHENSIVE TEST
        let invalid_char_list = vec!["\"", "$", "&", "?", "\'"];
        for invalid_char in invalid_char_list {
            assert!(matches!(lex(invalid_char), Err(_)));
        }
    }

    #[test]
    fn simple_chars() {
        //basic test for char ordering
        test_lex!("(", [LeftParen]);
        test_lex!("( 1", [LeftParen, Num(1)]);
        test_lex!("1 (", [Num(1), LeftParen]);

        //basic test for single chars
        test_lex!(")", [RightParen]);
        test_lex!("{", [LeftCurly]);
        test_lex!("}", [RightCurly]);
        test_lex!("[", [LeftBracket]);
        test_lex!("]", [RightBracket]);
        test_lex!(",", [Comma]);
        test_lex!(";", [Semicolon]);
        test_lex!("+", [Plus]);
        test_lex!("-", [Subtract]);
        test_lex!("*", [Multiply]);
        test_lex!("/", [Divide]);
        test_lex!("%", [Modulus]);
    }

    #[test]
    fn complex_chars() {
        test_lex!("<", [Less]);
        test_lex!("<=", [LessEqual]);
        test_lex!(">", [Greater]);
        test_lex!(">=", [GreaterEqual]);
        test_lex!("=", [Assign]);
        test_lex!("==", [Equality]);
        test_lex!("!=", [NotEqual]);
    }

    #[test]
    fn keywords() {
        //string order test
        test_lex!("func", [Func]);
        test_lex!("func 1", [Func, Num(1)]);
        test_lex!("1 func", [Num(1), Func]);

        //basic string test
        test_lex!("return", [Return]);
        test_lex!("int", [Int]);
        test_lex!("print", [Print]);
        test_lex!("read", [Read]);
        test_lex!("while", [While]);
        test_lex!("if", [If]);
        test_lex!("else", [Else]);
        test_lex!("break", [Break]);
        test_lex!("continue", [Continue]);
    }

    #[test]
    fn variable_names() {
        test_lex!("myVariable", [Ident("myVariable".to_string())]);
        test_lex!("myVariable", [Ident("myVariable".to_string())]);
    }

    #[test]
    fn dummy_code() {
        
    }
}

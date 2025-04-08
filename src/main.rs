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
                //plus
                tokens.push(Token::Plus);
                i += 1;
            }

            '-' => {
                //subtract
                tokens.push(Token::Subtract);
                i += 1;
            }

            '*' => {
                //multiply
                tokens.push(Token::Multiply);
                i += 1;
            }

            '/' => {
                //divide
                tokens.push(Token::Divide);
                i += 1;
            }

            '%' => {
                //modulus
                tokens.push(Token::Modulus);
                i += 1;
            }

            '=' => {
                //assign
                tokens.push(Token::Assign);
                i += 1;
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

                    _ => {
                        //word is a variable name
                        println!("This is an identifier: {word}");
                        tokens.push(Token::Ident(String::from(word)));
                    }
                }
            }

            //spaces | newlines -> ignore
            ' ' | '\n' => i += 1,

            '#' => {
                //comments
                let start = i;
                while i < bytes.len() {
                    let comment_char = bytes[i] as char;
                    if comment_char == '\n' {
                        break;
                    } else {
                        i += 1;
                    }
                }
                let end = i;
                let comment_token = &code[start..end];
                println!("This is a comment: {}", comment_token);
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
            ':' => {
                tokens.push(Token::Semicolon);
                i += 1;
            }

            _ => {
                //Error case
                return Err(format!("Unrecognized symbol '{}'", c));
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

    macro_rules! test_lex {//template for testing
        ($input:expr, [$( $token:expr ),* $(,)?]) => {
            {
                use Token::*; 
                let tokens = lex($input).unwrap();
                let expected = vec![$($token),*, End];
                assert_eq!(tokens, expected, "Failed on input: {}", $input);
            }
        };
    }

    #[test]
    fn lexer_tests() {
        test_lex!("1 + 2 + 3", [Num(1), Plus, Num(2), Plus, Num(3)]);

    }

    // #[test]
    // fn plus_tests() {
    //     // plus between numbers
    //     let toks = lex("1 + 2 + 3").unwrap();
    //     assert!(toks.len() == 6);
    //     assert!(matches!(toks[0], Token::Num(1)));
    //     assert!(matches!(toks[1], Token::Plus));
    //     assert!(matches!(toks[2], Token::Num(2)));
    //     assert!(matches!(toks[3], Token::Plus));
    //     assert!(matches!(toks[4], Token::Num(3)));
    //     assert!(matches!(toks[5], Token::End));


    //     let inputs: Vec<Token> = 

    //     //plus at the end
    //     // let toks = lex("3 + 215 +").unwrap();
    //     assert!(toks.len() == 5);
    //     assert!(matches!(toks[0], Token::Num(3)));
    //     assert!(matches!(toks[1], Token::Plus));
    //     assert!(matches!(toks[2], Token::Num(215)));
    //     assert!(matches!(toks[3], Token::Plus));
    //     assert!(matches!(toks[4], Token::End));

    //     //plus at the beginning
    //     let toks = lex("+ 7 + 5").unwrap();
    //     assert!(toks.len() == 5);
    //     assert!(matches!(toks[1], Token::Plus));
    //     assert!(matches!(toks[2], Token::Num(7)));
    //     assert!(matches!(toks[3], Token::Plus));
    //     assert!(matches!(toks[0], Token::Num(5)));
    //     assert!(matches!(toks[4], Token::End));

    //     //no spaces
    //     let toks = lex("1+2+3").unwrap();
    //     assert!(toks.len() == 6);
    //     assert!(matches!(toks[0], Token::Num(1)));
    //     assert!(matches!(toks[1], Token::Plus));
    //     assert!(matches!(toks[2], Token::Num(2)));
    //     assert!(matches!(toks[3], Token::Plus));
    //     assert!(matches!(toks[4], Token::Num(3)));
    //     assert!(matches!(toks[5], Token::End));
    // }
}

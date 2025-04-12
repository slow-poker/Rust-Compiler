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
use strum_macros::Display;
#[derive(Debug, PartialEq, Display)] //clone
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

//used only for testing
#[allow(dead_code)]
impl Token {
    pub fn to_file_string(&self) -> String {
        match self {
            Token::Num(number) => format!("Num({})", number),
            Token::Ident(code_string) => format!("Ident(\"{}\")", code_string),
            _ => format!("{}", self),
        }
    }
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
                    let symbol = bytes[i] as char;
                    if (symbol >= 'a' && symbol <= 'z') || (symbol >= 'A' && symbol <= 'Z') || (symbol >= '0' && symbol <= '9'){
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

    //helper function
    macro_rules! test_lex_eq {//template for testing
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
        test_lex_eq!("(", [LeftParen]);
        test_lex_eq!("( 1", [LeftParen, Num(1)]);
        test_lex_eq!("1 (", [Num(1), LeftParen]);

        //basic test for single chars
        test_lex_eq!(")", [RightParen]);
        test_lex_eq!("{", [LeftCurly]);
        test_lex_eq!("}", [RightCurly]);
        test_lex_eq!("[", [LeftBracket]);
        test_lex_eq!("]", [RightBracket]);
        test_lex_eq!(",", [Comma]);
        test_lex_eq!(";", [Semicolon]);
        test_lex_eq!("+", [Plus]);
        test_lex_eq!("-", [Subtract]);
        test_lex_eq!("*", [Multiply]);
        test_lex_eq!("/", [Divide]);
        test_lex_eq!("%", [Modulus]);
    }

    #[test]
    fn complex_chars() {
        test_lex_eq!("<", [Less]);
        test_lex_eq!("<=", [LessEqual]);
        test_lex_eq!(">", [Greater]);
        test_lex_eq!(">=", [GreaterEqual]);
        test_lex_eq!("=", [Assign]);
        test_lex_eq!("==", [Equality]);
        test_lex_eq!("!=", [NotEqual]);
    }

    #[test]
    fn keywords() {
        //string order test
        test_lex_eq!("func", [Func]);
        test_lex_eq!("func 1", [Func, Num(1)]);
        test_lex_eq!("1 func", [Num(1), Func]);

        //basic string test
        test_lex_eq!("return", [Return]);
        test_lex_eq!("int", [Int]);
        test_lex_eq!("print", [Print]);
        test_lex_eq!("read", [Read]);
        test_lex_eq!("while", [While]);
        test_lex_eq!("if", [If]);
        test_lex_eq!("else", [Else]);
        test_lex_eq!("break", [Break]);
        test_lex_eq!("continue", [Continue]);
    }

    #[test]
    fn variable_names() {
        test_lex_eq!("myVariable", [Ident("myVariable".to_string())]);
        test_lex_eq!(" myVariable", [Ident("myVariable".to_string())]);
        test_lex_eq!("myVariable ", [Ident("myVariable".to_string())]);
        test_lex_eq!("my2Variable", [Ident("my2Variable".to_string())]);
        test_lex_eq!("123myVariable", [Num(123), Ident("myVariable".to_string())]);
        test_lex_eq!("myVariable123", [Ident("myVariable123".to_string())]);
    }

    #[test]
    fn example_code() {
        use std::fs::{DirEntry, read_to_string, read_dir};
        use relative_path::RelativePath;

        // Create and convert the relative path
        let rel_path_ex = RelativePath::new("examples");
        let pathbuf_ex = rel_path_ex.to_path("./");
        let paths_ex = read_dir(pathbuf_ex).unwrap();

        let mut test_files: Vec<DirEntry> = vec![];
        for file in paths_ex {
            test_files.push(file.unwrap());
        }

        let mut i = 0;
        while i < test_files.len() {
            if test_files[i].path().display().to_string().ends_with(".tt") {

                //create test_token_string
                let lex_input = read_to_string(test_files[i].path()).unwrap();
                let test_token_vec: Vec<String> = lex(&lex_input).unwrap().iter().map(|t| t.to_file_string()).collect();
                let test_token_string = test_token_vec.join("\n") + "\n"; //sol files have trailing newline

                //find matching sol file
                let test_path_string= test_files[i].path().display().to_string();
                let mut sol_filename = "".to_string();
                for (i, c) in test_path_string.char_indices() {
                    if c == '/' {
                        let test_filename = test_path_string[i..test_path_string.len()-3].to_string() + "Sol";
                        sol_filename = test_path_string[0..i].to_string() + "/solutions" + &test_filename;
                    }
                }

                //create sol_token_string
                let sol_token_string = read_to_string(std::path::Path::new(&sol_filename)).unwrap();
                assert_eq!(test_token_string, sol_token_string); //add match for error with str "check trailing newlines in sol files"
            }
            i+=1;
        }
    }
}

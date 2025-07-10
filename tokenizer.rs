//JULIAN CHAOUL 
//ASSIGNMENT 8 
//4/28/2025
mod common; 
use common::{Lex, Token};
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::env;


fn main() -> io::Result<()>{
    //Retrieve command line arguments 
    let args : Vec<String> = env::args().collect();

    //If less than 3 arguments are inputted
    if args.len() < 3 {
        eprintln!("Wrong input");
        std::process::exit(1);
    }
    
    // Extract the input and output fle 
    let inf = File::open(&args[1])?;
    let reader = BufReader::new(inf);
    let mut outf = File::create(&args[2])?;


    // Store tokens within the vector 
    let mut tokens = Vec::new();
    
    //Read input file
    for line in reader.lines() {
        //tokenize each line 
        tokens.extend(tokenizeline(&line?));
    }

    // Identify and write each token and its type to the output file
    for lexeme in tokens.iter() {
        let tok = identify(lexeme); // Use identify function below 
        writeln!(outf, "{:?} {}", tok.token, tok.lexeme)?; // Output format: TokenType lexeme
    }

     Ok(())
}

fn tokenizeline(line: &str) -> Vec<String> { // tokenize a single line of code into Lexemes 
    let mut lexemes = Vec::new(); // create new vector 
    let chars: Vec<char> = line.chars().collect(); // convert line into vector of characters
    let mut ii = 0; //iterator
    while ii <chars.len(){
        let ch = chars[ii];

        if ch.is_whitespace() { // skip the whitespaces
            ii += 1;
            continue;
        }
        // handle two character operations such as != and == 
        if ii + 1 <chars.len(){
            let fig = format!("{}{}", chars[ii], chars[ii + 1]);
            if["!=", "=="].contains(&fig.as_str()) {
                lexemes.push(fig);
                ii+=2;
                continue;
            }
        }
        // handle the single character tokens
        if "(){}=,;+*%".contains(ch) {
            lexemes.push(ch.to_string());
            ii += 1;
            continue;
        }
        // handle identifiers using the ascii alphabet
        if ch.is_ascii_alphabetic() {
            let mut lexeme = String::new();
            while ii < chars.len() && chars[ii].is_ascii_alphanumeric() {
                lexeme.push(chars[ii]);
                ii += 1;
            }
            lexemes.push(lexeme);
            continue;
        }
        // Handle numbers
        if ch.is_ascii_digit() {
            let mut lexeme = String::new();
            while ii < chars.len() && chars[ii].is_ascii_digit() {
                lexeme.push(chars[ii]);
                ii += 1;
            }
            lexemes.push(lexeme);
            continue;
        }
        
        ii += 1; // will help skip unknown characrers 
    }
    lexemes
}
// Function to help identify 
fn identify(lexeme: &str) -> Lex {
    let token = match lexeme {
        // Parentheses
        "(" => Token::LEFT_PARENTHESIS,
        ")" => Token::RIGHT_PARENTHESIS,
        // Curly brackets 
        "{" => Token::LEFT_BRACKET,
        "}" => Token::RIGHT_BRACKET,
        // Keywords
        "while" => Token::WHILE_KEYWORD,
        "return" => Token::RETURN_KEYWORD,
        "int" | "void" => Token::VARTYPE,
        // Assignment operator
        "=" => Token::EQUAL,
        // Comma and semi-colon
        "," => Token::COMMA,
        ";" => Token::EOL,
        // Binary operators (arithmetic, comparison)
        "+" | "*" | "%" | "!=" | "==" => Token::BINOP,
        // will handle number literals 
        _ if lexeme.chars().all(|c| c.is_ascii_digit()) => Token::NUMBER,
        // fallback assuming its an identifier 
        _ => Token::IDENTIFIER,
    };
    Lex { token, lexeme: lexeme.to_string() } // In order to return a type Lex
}
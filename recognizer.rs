//JULIAN CHAOUL 
//ASSIGNMENT 8 
//4/28/2025

mod common;
use common::{Lex, Token};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};


fn main() {
    // Retrieve command line arguments 
    let args: Vec<String> = env::args().collect();
    // Need exactly two arguments 
    if args.len() != 3 {
        eprintln!("Usage: recognizer <token_file> <output_file>");
        std::process::exit(1);
    }

    // Read tokens from input file and parse
    let tokens = read_tokens(&args[1]).expect("Failed to read token file");
    let mut parser = Parser::new(tokens, &args[2]); // initialize parser
    parser.parse(); // start the parcing process 
}


struct Parser { // struct for parser
    tokens: Vec<Lex>,  // List of tokens 
    pos: usize,        // Current position 
    output: File,      // Output file 
}

impl Parser {
    // Create a new parser instance
    fn new(tokens: Vec<Lex>, output_path: &str) -> Self {
        let output = File::create(output_path).expect("Cannot open output file");
        Parser { tokens, pos: 0, output }
    }

    
    fn error(&mut self, msg: &str) -> ! {
        writeln!(self.output, "{}", msg).unwrap(); // if error found will handle erro
        std::process::exit(0); // exit 
    }

    // Peeks at the next token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|l| &l.token)
    }

    // Consumes a token if it matches the expected one; otherwise errors
    fn consume(&mut self, expected: Token, rule: &str) -> bool {
        if let Some(token) = self.peek() { // using peek method, check token
            if *token == expected {
                self.pos += 1; // if it is expected it will iterate 
                true
            } else { // handle errors
                self.error(&format!( 
                    "Error: In grammar rule {}, expected token #{} to be {:?} but was {:?}",
                    rule, self.pos + 1, expected, token
                ));
            }
        } else { // handle errrors
            self.error(&format!( 
                "Error: In grammar rule {}, expected token #{} to be {:?} but input ended",
                rule, self.pos + 1 , expected
            ));
        }
    }

    // Begins parsing
    fn parse(&mut self) {
        self.function(); //start at function 

        // Check if all tokens were consumed
        if self.pos < self.tokens.len() {
            self.error(&format!( // if not consumed throw error
                "Error: Only consumed {} of the {} given tokens",
                self.pos, self.tokens.len()
            ));
        }

        // Write success message
        writeln!(self.output, "PARSED!!!").unwrap();
    }
    
    // Grammar: function --> header body
    fn function(&mut self) {
        if self.peek() == Some(&Token::VARTYPE) {
            self.header();
            self.body();
            
        } else {
            self.error("Error: In grammar rule function, expected a valid header non-terminal to be present but was not.")
        }
    }
    // Will use consume to return various points of grammer, each assigned something depended on name of grammer 
    // Grammar: header --> VARTYPE IDENTIFIER ( [arg-decl] )
    fn header(&mut self) {
        self.consume(Token::VARTYPE, "header");
        self.consume(Token::IDENTIFIER, "header");
        self.consume(Token::LEFT_PARENTHESIS, "header");
        self.arg_decl();
        self.consume(Token::RIGHT_PARENTHESIS, "header");
    }

    // Grammar: arg-decl --> VARTYPE IDENTIFIER { COMMA VARTYPE IDENTIFIER }
    fn arg_decl(&mut self) {
        if self.peek() == Some(&Token::VARTYPE) {
            self.consume(Token::VARTYPE, "arg-decl");
            self.consume(Token::IDENTIFIER, "arg-decl");
            while self.peek() == Some(&Token::COMMA) {
                self.consume(Token::COMMA, "arg-decl");
                self.consume(Token::VARTYPE, "arg-decl");
                self.consume(Token::IDENTIFIER, "arg-decl");
            }
        }
    }

    // Grammar: body --> { [statement-list] }
    fn body(&mut self) {
        self.consume(Token::LEFT_BRACKET, "body");
        while self.peek().is_some()
            && matches!(self.peek().unwrap(), Token::WHILE_KEYWORD | Token::RETURN_KEYWORD | Token::IDENTIFIER)
        {
            self.statement();
        }
        self.consume(Token::RIGHT_BRACKET, "body");
    }

    // Grammar: statement --> while-loop | return | assignment
    fn statement(&mut self) {
        match self.peek() {
            Some(Token::WHILE_KEYWORD) => self.while_loop(),
            Some(Token::RETURN_KEYWORD) => self.return_stmt(),
            Some(Token::IDENTIFIER) => self.assignment(),
            _ => self.error("Error: In grammar rule statement, expected a valid statement non-terminal to be present but was not."),
        }
    }

    // Grammar: while-loop --> while ( expression ) body
    fn while_loop(&mut self) {
        self.consume(Token::WHILE_KEYWORD, "while-loop");
        self.consume(Token::LEFT_PARENTHESIS, "while-loop");
        self.expression();
        self.consume(Token::RIGHT_PARENTHESIS, "while-loop");
        self.body();
    }

    // Grammar: return --> return expression ;
    fn return_stmt(&mut self) {
        self.consume(Token::RETURN_KEYWORD, "return");
        self.expression();
        self.consume(Token::EOL, "return");
    }

    // Grammar: assignment --> IDENTIFIER = expression ;
    fn assignment(&mut self) {
        self.consume(Token::IDENTIFIER, "assignment");
        self.consume(Token::EQUAL, "assignment");
        self.expression();
        self.consume(Token::EOL, "assignment");
    }

    // Grammar: expression --> term { BINOP term } | ( expression )
    fn expression(&mut self) {
        if self.peek() == Some(&Token::LEFT_PARENTHESIS) {
            self.consume(Token::LEFT_PARENTHESIS, "expression");
            self.expression();
            self.consume(Token::RIGHT_PARENTHESIS, "expression");
        } else {
            self.term();
            while self.peek() == Some(&Token::BINOP) {
                self.consume(Token::BINOP, "expression");
                self.term();
            }
        }
    }

    // Grammar: term --> IDENTIFIER | NUMBER
    fn term(&mut self) {
        match self.peek() {
            Some(Token::IDENTIFIER) | Some(Token::NUMBER) => {
                self.pos += 1;
            }
            _ => self.error("Error: In grammar rule term, expected IDENTIFIER or NUMBER"),
        }
    }
}

// Will read tokens from file 
fn read_tokens(path: &str) -> io::Result<Vec<Lex>> {
    // Initialize file, reader, and tokens 
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut tokens = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((token_str, lexeme)) = line.split_once(' ') {
            let token = match token_str {
                "LEFT_PARENTHESIS" => Token::LEFT_PARENTHESIS,
                "RIGHT_PARENTHESIS" => Token::RIGHT_PARENTHESIS,
                "LEFT_BRACKET" => Token::LEFT_BRACKET,
                "RIGHT_BRACKET" => Token::RIGHT_BRACKET,
                "WHILE_KEYWORD" => Token::WHILE_KEYWORD,
                "RETURN_KEYWORD" => Token::RETURN_KEYWORD,
                "EQUAL" => Token::EQUAL,
                "COMMA" => Token::COMMA,
                "EOL" => Token::EOL,
                "VARTYPE" => Token::VARTYPE,
                "IDENTIFIER" => Token::IDENTIFIER,
                "BINOP" => Token::BINOP,
                "NUMBER" => Token::NUMBER,
                _ => continue,
            };
            tokens.push(Lex { token, lexeme: lexeme.to_string() });
        }
    }

    Ok(tokens)
}

//JULIAN CHAOUL 
//ASSIGNMENT 8 
//4/28/2025
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LEFT_PARENTHESIS, 
    RIGHT_PARENTHESIS, 
    LEFT_BRACKET, 
    RIGHT_BRACKET, 
    WHILE_KEYWORD, 
    RETURN_KEYWORD, 
    EQUAL, 
    COMMA, 
    EOL, 
    VARTYPE,
    IDENTIFIER, 
    BINOP, 
    NUMBER,
}

#[derive(Debug, Clone)]
pub struct Lex { 
    pub token: Token,
    pub lexeme: String,
}

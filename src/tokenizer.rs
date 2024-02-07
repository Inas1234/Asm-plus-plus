use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    Mov,
    Add,
    Lparen,
    Rparen,
    Comma,
    Global,
    Function,
    CurlyL,
    CurlyR,
    Syscall,
    Call,    
    Section,
    Colon,
    StringLit,
    Equal,
    Lesser,
    Greater,
    NotEqual,
    If,
    While,
    Push,
    Xor,
    Pop,
    Len
}


#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

fn token_type_to_String(token_type: TokenType) -> String {
    match token_type {
        TokenType::Number => "Number".to_string(),
        TokenType::Identifier => "Identifier".to_string(),
        TokenType::Mov => "Mov".to_string(),
        TokenType::Add => "Add".to_string(),
        TokenType::Lparen => "Lparen".to_string(),
        TokenType::Rparen => "Rparen".to_string(),
        TokenType::Comma => "Comma".to_string(),
        TokenType::Global => "Global".to_string(),
        TokenType::Function => "Function".to_string(),
        TokenType::CurlyL => "CurlyL".to_string(),
        TokenType::CurlyR => "CurlyR".to_string(),
        TokenType::Syscall => "Syscall".to_string(),
        TokenType::Call => "Call".to_string(),
        TokenType::Section => "Section".to_string(),
        TokenType::Colon => "Colon".to_string(),
        TokenType::StringLit => "StringLit".to_string(),
        TokenType::Equal => "Equal".to_string(),
        TokenType::Lesser => "Lesser".to_string(),
        TokenType::Greater => "Greater".to_string(),
        TokenType::NotEqual => "NotEqual".to_string(),
        TokenType::If => "If".to_string(),
        TokenType::While => "While".to_string(),
        TokenType::Push => "Push".to_string(),
        TokenType::Xor => "Xor".to_string(),
        TokenType::Pop => "Pop".to_string(),
        TokenType::Len => "Len".to_string(),
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({:?}, {:?})", token_type_to_String(self.token_type), self.value).expect("Error in formatting");
        Ok(())
    }
}

pub struct Tokenizer {
    contents: String,
    index: usize,
}

impl Tokenizer {
    pub fn new(contents: String) -> Tokenizer {
        Tokenizer {
            contents,
            index: 0,
        }
    }


    pub fn tokenize(&mut self) -> Vec<Token>{
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        while let Some(c) = self.peek(0) {
            if c == ';' {
                while let Some(c) = self.peek(0) {
                    if c != '\n' {
                        self.consume();
                    } else {
                        break;
                    }
                }
            }
            else if c.is_alphabetic() || c == '_'{
                buffer.push(self.consume());
                while let Some(c) = self.peek(0) {
                    if c.is_alphanumeric() || c == '_' {
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                }

                match buffer.as_str() {
                    "mov" => tokens.push(Token { token_type: TokenType::Mov, value: None }),
                    "add" => tokens.push(Token { token_type: TokenType::Add, value: None }),
                    "global" => tokens.push(Token { token_type: TokenType::Global, value: None }),
                    "fn" => tokens.push(Token { token_type: TokenType::Function, value: None }),
                    "syscall" => tokens.push(Token { token_type: TokenType::Syscall, value: None }),
                    "call" => tokens.push(Token { token_type: TokenType::Call, value: None }),
                    "section" => tokens.push(Token { token_type: TokenType::Section, value: None }),
                    "eq" => tokens.push(Token { token_type: TokenType::Equal, value: None }),
                    "lt" => tokens.push(Token { token_type: TokenType::Lesser, value: None }),
                    "gt" => tokens.push(Token { token_type: TokenType::Greater, value: None }),
                    "ne" => tokens.push(Token { token_type: TokenType::NotEqual, value: None }),
                    "if" => tokens.push(Token { token_type: TokenType::If, value: None }),
                    "while" => tokens.push(Token { token_type: TokenType::While, value: None }),
                    "push" => tokens.push(Token { token_type: TokenType::Push, value: None }),
                    "xor" => tokens.push(Token { token_type: TokenType::Xor, value: None }),
                    "pop" => tokens.push(Token { token_type: TokenType::Pop, value: None }),
                    "len" => tokens.push(Token { token_type: TokenType::Len, value: None }),
                    _ => tokens.push(Token { token_type: TokenType::Identifier, value: Some(buffer.clone()) }),
                }
                buffer.clear();
            }
            else if c.is_digit(10){
                buffer.push(self.consume());
                while let Some(c) = self.peek(0) {
                    if c.is_digit(10) {
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                }
                tokens.push(Token { token_type: TokenType::Number, value: Some(buffer.clone()) });
                buffer.clear();
            }
            else if c == '"' {
                self.consume();
                while let Some(c) = self.peek(0) {
                    if c != '"' {
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                }
                self.consume();
                tokens.push(Token { token_type: TokenType::StringLit, value: Some(buffer.clone()) });
                buffer.clear();
            }
            else if c == '(' {
                tokens.push(Token { token_type: TokenType::Lparen, value: None });
                self.consume();
            }
            else if c == ')' {
                tokens.push(Token { token_type: TokenType::Rparen, value: None });
                self.consume();
            }
            else if c == ',' {
                tokens.push(Token { token_type: TokenType::Comma, value: None });
                self.consume();
            }
            else if c == '{' {
                tokens.push(Token { token_type: TokenType::CurlyL, value: None });
                self.consume();
            }
            else if c == '}' {
                tokens.push(Token { token_type: TokenType::CurlyR, value: None });
                self.consume();
            }
            else if c == ':' {
                tokens.push(Token { token_type: TokenType::Colon, value: None });
                self.consume();
            }
            else if c.is_whitespace() {
                self.consume();
            }
            else {
                self.consume();
            }
        }
        self.index = 0;
        tokens
    } 


    fn peek(&self, ahead: usize) -> Option<char> {
        let index = self.index + ahead;
        if index < self.contents.len() {
            self.contents.chars().nth(index)
        } else {
            None
        }
    }

    fn consume(&mut self) -> char {
        if self.index < self.contents.len(){
            let c = self.contents[self.index..].chars().next().unwrap();
            self.index += 1;
            c
        }
        else {
            '\0'
        }
    }
}


use crate::tokenizer::{self, Token};

#[derive(Debug)]
pub struct NodeExprIdent {
    pub name: String,
}

#[derive(Debug)]
pub struct NodeExprNumber {
    pub value: i32,
}

#[derive(Debug)]
pub struct NodeExprString {
    pub value: String,
}

#[derive(Debug)]
pub enum NodeExpr {
    Ident(NodeExprIdent),
    Number(NodeExprNumber),
    String(NodeExprString),
}

impl From<NodeExprIdent> for NodeExpr {
    fn from(ident: NodeExprIdent) -> Self {
        NodeExpr::Ident(ident)
    }
}

impl From<NodeExprNumber> for NodeExpr {
    fn from(number: NodeExprNumber) -> Self {
        NodeExpr::Number(number)
    }
}

impl From<NodeExprString> for NodeExpr {
    fn from(string: NodeExprString) -> Self {
        NodeExpr::String(string)
    }
}

#[derive(Debug)]
pub struct NodeStmtMov {
    pub ident: NodeExprIdent,
    pub expr: NodeExpr,
}

#[derive(Debug)]
pub struct NodeStmtAdd {
    pub ident: NodeExprIdent,
    pub expr: NodeExpr,
}

#[derive(Debug)]
pub struct NodeStmtGlobal {
    pub ident: NodeExprIdent,
}

#[derive(Debug)]
pub struct NodeFunc {
    pub name: NodeExprIdent,     
    pub arguments: Vec<NodeExprIdent>, 
    pub body: Vec<NodeStmt>,     
}

#[derive(Debug)]
pub struct NodeStmtSyscall {}

#[derive(Debug)]
pub struct NodeStmtCall {
    pub name: NodeExprIdent,
    pub arguments: Vec<NodeExpr>,
}

#[derive(Debug)]
pub struct NodeStmtSection {
    pub name: NodeExprIdent,
}

#[derive(Debug)]
pub struct NodeStmtAssign {
    pub ident: NodeExprIdent,
    pub expr: NodeExpr,
}



#[derive(Debug)]
pub enum NodeStmt {
    Mov(NodeStmtMov),
    Add(NodeStmtAdd),
    Global(NodeStmtGlobal),
    Func(NodeFunc),
    Syscall(NodeStmtSyscall),
    Call(NodeStmtCall),
    Section(NodeStmtSection),
    Assign(NodeStmtAssign),
}

#[derive(Debug)]
pub struct Node {
    pub functions: Vec<NodeFunc>,
    pub stmt: Vec<NodeStmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            index: 0,
        }
    }

    pub fn parse_expression(&mut self) -> NodeExpr {
        let token = self.consume().unwrap();
        match token.token_type {
            tokenizer::TokenType::Number => {
                NodeExpr::Number(NodeExprNumber {
                    value: token.value.clone().unwrap().parse().unwrap(),
                })
            }
            tokenizer::TokenType::Identifier => {
                NodeExpr::Ident(NodeExprIdent {
                    name: token.value.clone().unwrap(),
                })
            }
            tokenizer::TokenType::StringLit => {
                NodeExpr::String(NodeExprString {
                    value: token.value.clone().unwrap(),
                })
            }
            _ => panic!("Unexpected token {:?}", token),
        }
    }

    fn parse_mov(&mut self) -> NodeStmt {
        self.consume();
        let ident_expr = self.parse_expression();
        let _ = self.consume().unwrap();
        
        if let NodeExpr::Ident(ident) = ident_expr {
            let expr = self.parse_expression();
            NodeStmt::Mov(NodeStmtMov { ident: ident, expr })
        } else {
            panic!("Expected an identifier in the 'mov' statement.");
        }
    }

    fn parse_add(&mut self) -> NodeStmt {
        self.consume();
        let ident_expr = self.parse_expression();
        let _ = self.consume().unwrap();
        
        if let NodeExpr::Ident(ident) = ident_expr {
            let expr = self.parse_expression();
            NodeStmt::Add(NodeStmtAdd { ident: ident, expr })
        } else {
            panic!("Expected an identifier in the 'add' statement.");
        }
    }

    fn parse_global(&mut self) -> NodeStmt {
        self.consume();
        let ident_expr = self.parse_expression();
        
        if let NodeExpr::Ident(ident) = ident_expr {
            NodeStmt::Global(NodeStmtGlobal { ident: ident })
        } else {
            panic!("Expected an identifier in the 'global' statement.");
        }
    }

    fn parse_function(&mut self) -> NodeStmt {
        self.consume(); // Consume the "func" keyword
        let name = match self.parse_expression() {
            NodeExpr::Ident(ident) => ident,
            _ => panic!("Expected an identifier for the function name."),
        };

        let opening_bracket = self.consume().unwrap(); 
        if opening_bracket.token_type != tokenizer::TokenType::Lparen {
            panic!("Expected a lparen token to open the function arguments.");
        }

        let mut arguments = Vec::new();
        while let Some(token) = self.peek(0) {
            match token.token_type {
                tokenizer::TokenType::Identifier => {
                    arguments.push(match self.parse_expression() {
                        NodeExpr::Ident(ident) => ident,
                        _ => panic!("Expected an identifier for an argument."),
                    });
                    
                }
                tokenizer::TokenType::Comma => {
                    let _ = self.consume().unwrap();
                }
                _ => break, 
            }
        }
        let closing_bracket = self.consume().unwrap(); 
        if closing_bracket.token_type != tokenizer::TokenType::Rparen {
            panic!("Expected a rparen token to close the function arguments.");
        }

        let opening_curly = self.consume().unwrap(); 

        if opening_curly.token_type != tokenizer::TokenType::CurlyL {
            panic!("Expected a curlyL token to open the function body.");
        }

        let mut body = Vec::new();
        while let Some(token) = self.peek(0) {
            match token.token_type {
                tokenizer::TokenType::Mov
                | tokenizer::TokenType::Add
                | tokenizer::TokenType::Global 
                | tokenizer::TokenType::Syscall 
                | tokenizer::TokenType::Call 
                | tokenizer::TokenType::Section => {
                    body.push(self.parse_statment().unwrap());
                }
                _ => break, 
            }
        }

        let closing_curly= self.consume().unwrap();
        if closing_curly.token_type != tokenizer::TokenType::CurlyR {
            panic!("Expected a curlyR token to close the function body.");
        }

            

        NodeStmt::Func(NodeFunc {
            name,
            arguments,
            body,
        })
    }

    fn parse_syscall(&mut self) -> NodeStmt {
        self.consume();
        NodeStmt::Syscall(NodeStmtSyscall {})
    }


    fn parse_call(&mut self) -> NodeStmt {
        self.consume();
        let name = match self.parse_expression() {
            NodeExpr::Ident(ident) => ident,
            _ => panic!("Expected an identifier for the function name."),
        };

        let _ = self.consume().unwrap(); 
        let mut arguments = Vec::new();
        while let Some(token) = self.peek(0) {
            match token.token_type {
                tokenizer::TokenType::Number
                | tokenizer::TokenType::Identifier => {
                    arguments.push(self.parse_expression());
                    if let Some(token) = self.peek(0) {
                        match token.token_type {
                            tokenizer::TokenType::Comma => {
                                let _ = self.consume().unwrap();
                            }
                            _ => break,
                        }
                    }
                }
                _ => break,
            }
        }
        let _ = self.consume().unwrap(); 
        NodeStmt::Call(NodeStmtCall { name, arguments })
    }


    fn parse_section(&mut self) -> NodeStmt {
        self.consume();
        let name = match self.parse_expression() {
            NodeExpr::Ident(ident) => ident,
            _ => panic!("Expected an identifier for the section name."),
        };
        NodeStmt::Section(NodeStmtSection { name })
    }   

    fn parse_assign(&mut self) -> NodeStmt {
        let ident = match self.parse_expression() {
            NodeExpr::Ident(ident) => ident,
            _ => panic!("Expected an identifier for the assignment."),
        };
        let _ = self.consume().unwrap(); 
        let expr = self.parse_expression();
        NodeStmt::Assign(NodeStmtAssign { ident, expr })
    }

    pub fn parse_statment(&mut self) -> Option<NodeStmt> {
        while let Some(token) = self.peek(0)  {
            match token.token_type {
                tokenizer::TokenType::Mov => {
                    return Some(self.parse_mov());
                }
                tokenizer::TokenType::Add => {
                    return Some(self.parse_add());
                }
                tokenizer::TokenType::Global => {
                    return Some(self.parse_global());
                }
                tokenizer::TokenType::Syscall => {
                    return Some(self.parse_syscall());
                }
                tokenizer::TokenType::Function => {
                    return Some(self.parse_function());
                }
                tokenizer::TokenType::Call => {
                    return Some(self.parse_call());
                }
                tokenizer::TokenType::Section => {
                    return Some(self.parse_section());
                }
                tokenizer::TokenType::Identifier => {
                    return Some(self.parse_assign());
                }
                _ => {
                    panic!("Unexpected token {:?}", token);
                }
                
            }
            
        }
        None
    }

    pub fn parse_prog(&mut self) -> Node {
        let mut stmt = Vec::new();
        let mut functions = Vec::new();

        while let Some(node) = self.parse_statment() {
            match node {
                NodeStmt::Func(func) => functions.push(func),
                _ => stmt.push(node),
            }
        }

        Node { stmt, functions }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        let index = self.index + offset;
        if index >= self.tokens.len() {
            None
        }
        else {
            Some(&self.tokens[index])
        }
    }
    
    fn consume(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() {
            let c = self.tokens.get(self.index);
            self.index += 1;
            c
        }
        else {
            None
        }
    }
}

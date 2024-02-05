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
pub enum NodeExpr {
    Ident(NodeExprIdent),
    Number(NodeExprNumber),
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
pub enum NodeStmt {
    Mov(NodeStmtMov),
    Add(NodeStmtAdd),
    Global(NodeStmtGlobal),
    Func(NodeFunc),
    Syscall(NodeStmtSyscall),
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

        let _ = self.consume().unwrap(); // Consume the '(' token
        let mut arguments = Vec::new();
        while let Some(token) = self.peek(0) {
            match token.token_type {
                tokenizer::TokenType::Identifier => {
                    arguments.push(match self.parse_expression() {
                        NodeExpr::Ident(ident) => ident,
                        _ => panic!("Expected an identifier for an argument."),
                    });
                    
                }
                _ => break, 
            }
        }
        let _ = self.consume().unwrap(); // Consume the ')' token

        let _ = self.consume().unwrap(); 

        let mut body = Vec::new();
        while let Some(token) = self.peek(0) {
            match token.token_type {
                tokenizer::TokenType::Mov
                | tokenizer::TokenType::Add
                | tokenizer::TokenType::Global 
                | tokenizer::TokenType::Syscall => {
                    body.push(self.parse_statment().unwrap());
                }
                _ => break, 
            }
        }

        let cirly= self.consume().unwrap();
        if cirly.token_type != tokenizer::TokenType::CurlyR {
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

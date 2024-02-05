use crate::parser::{Node, NodeStmt, NodeStmtMov, NodeStmtAdd, NodeExpr, NodeExprIdent, NodeExprNumber, NodeFunc};

pub struct Generator {
    node: Node,
}

impl Generator {
    pub fn new(node: Node) -> Generator {
        Generator {
            node,
        }
    }

    fn generate_statement(&self, stmt: &NodeStmt) -> String {
        match stmt {
            NodeStmt::Mov(mov) => {
                format!("  mov {}, {}\n", self.generate_expr_ident(&mov.ident), self.generate_expr(&mov.expr))
            }
            NodeStmt::Add(add) => {
                format!("  add {}, {}\n", self.generate_expr_ident(&add.ident), self.generate_expr(&add.expr))
            }
            NodeStmt::Global(global) => {
                format!("global {}\n", self.generate_expr_ident(&global.ident))
            }
            NodeStmt::Syscall(_syscall) => {
                "  syscall\n".to_string()
            }
            NodeStmt::Call(call) => {
                let mut result = format!("  call {}\n", self.generate_expr_ident(&call.name));
                for arg in &call.arguments {
                    result.push_str(&format!("  push {}\n", self.generate_expr(arg)));
                }
                result
            }
            _ => "".to_string(),
        }
    }

    fn generate_function(&self, func: &NodeFunc) -> String {
        let mut result = format!("{}:\n", self.generate_expr_ident(&func.name));

        for arg in &func.arguments {
            result.push_str(&format!("  ; Argument: {}\n", self.generate_expr_ident(arg)));
        }

        for stmt in &func.body {
            result.push_str(&self.generate_statement(stmt));
        }

        result.push_str("  ret\n");
        result
    }


    pub fn generate(&self) -> String {
        let mut result = String::new();

        for func in &self.node.functions {
            result.push_str(&self.generate_function(func));
        }

        for stmt in &self.node.stmt {
            result.push_str(&self.generate_statement(stmt));
        }
        result
    }

    fn generate_expr_ident(&self, ident: &NodeExprIdent) -> String {
        ident.name.clone()
    }

    fn generate_expr(&self, expr: &NodeExpr) -> String {
        match expr {
            NodeExpr::Ident(ident) => self.generate_expr_ident(ident),
            NodeExpr::Number(number) => self.generate_expr_number(number),
        }
    }

    fn generate_expr_number(&self, number: &NodeExprNumber) -> String {
        number.value.to_string()
    }
    
}
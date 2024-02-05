use crate::parser::{Node, NodeStmt, NodeStmtMov, NodeStmtAdd, NodeExpr, NodeExprIdent, NodeExprNumber};

pub struct Generator {
    node: Node,
}

impl Generator {
    pub fn new(node: Node) -> Generator {
        Generator {
            node,
        }
    }

    pub fn generate(&self) -> String {
        let mut result = String::new();
        for stmt in &self.node.stmt {
            match stmt {
                NodeStmt::Mov(mov) => {
                    result.push_str(&format!("mov {}, {}\n", self.generate_expr_ident(&mov.ident), self.generate_expr(&mov.expr)));
                }
                NodeStmt::Add(add) => {
                    result.push_str(&format!("add {}, {}\n", self.generate_expr_ident(&add.ident), self.generate_expr(&add.expr)));
                }
                NodeStmt::Global(global) => {
                    result.push_str(&format!("global {}\n", self.generate_expr_ident(&global.ident)));
                }
            }
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
use std::collections::HashMap;

use crate::parser::{Node, NodeStmt, NodeExpr, NodeExprIdent, NodeExprNumber, NodeFunc, NodeExprString};

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
                let mut result = String::new();

                let stack_space = call.arguments.len() * 8; 

                let reversed_args: Vec<_> = call.arguments.iter().rev().collect();

                for arg in reversed_args.iter() {
                    result.push_str(&format!("  push {}\n", self.generate_expr(arg)));
                }

                result.push_str(&format!("  call {}\n", self.generate_expr_ident(&call.name)));

                if stack_space > 0 {
                    result.push_str(&format!("  add rsp, {}\n", stack_space));
                }

                result
            }
            NodeStmt::Section(section) => {
                format!("section .{}\n", self.generate_expr_ident(&section.name))
            }
            NodeStmt::Assign(assign) => {
                format!("  {} db {} , 10\n", self.generate_expr_ident(&assign.ident), self.generate_expr(&assign.expr))
            }
            _ => "".to_string(),
        }
    }

    fn generate_function(&self, func: &NodeFunc) -> String {
        let mut result = format!("{}:\n", self.generate_expr_ident(&func.name));
        result.push_str("  push rbp\n");
        result.push_str("  mov rbp, rsp\n");

        let mut arg_stack_map: HashMap<String, String> = HashMap::new();
        for (index, arg) in func.arguments.iter().enumerate() {
            let arg_name = self.generate_expr_ident(arg);
            let stack_offset = (index + 2) * 8; 
            let stack_offset_str = format!("[rbp + {}]", stack_offset);
            arg_stack_map.insert(arg_name, stack_offset_str);
        }

        for stmt in &func.body {
            result.push_str(&self.generate_statement_with_arg_map(stmt, &arg_stack_map));
        }

        result.push_str("  mov rsp, rbp\n");
        result.push_str("  pop rbp\n");
        result.push_str("  ret\n");
        result
    }

    fn generate_statement_with_arg_map(&self, stmt: &NodeStmt, arg_register_map: &std::collections::HashMap<String, String>) -> String {
        match stmt {
            NodeStmt::Mov(mov) => {
                let target = self.generate_expr_ident(&mov.ident);
                let target_mapped = arg_register_map.get(&target).unwrap_or(&target);
                let value = self.generate_expr_with_arg_map(&mov.expr, arg_register_map);
                format!("  mov {}, {}\n", target_mapped, value)
            },
            NodeStmt::Add(add) => {
                let target = self.generate_expr_ident(&add.ident);
                let target_mapped = arg_register_map.get(&target).unwrap_or(&target);
                let value = self.generate_expr_with_arg_map(&add.expr, arg_register_map);
                format!("  add {}, {}\n", target_mapped, value)
            },
            _ => self.generate_statement(stmt),
        }
    }


    fn generate_expr_with_arg_map(&self, expr: &NodeExpr, arg_register_map: &std::collections::HashMap<String, String>) -> String {
        match expr {
            NodeExpr::Ident(ident) => {
                let name = self.generate_expr_ident(ident);
                arg_register_map.get(&name).unwrap_or(&name).clone()
            },
            NodeExpr::Number(number) => self.generate_expr_number(number),
            NodeExpr::String(string) => self.generate_string(string),
        }
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

    fn generate_string(&self, string: &NodeExprString) -> String {
        format!("\"{}\"", string.value)
    }

    fn generate_expr(&self, expr: &NodeExpr) -> String {
        match expr {
            NodeExpr::Ident(ident) => self.generate_expr_ident(ident),
            NodeExpr::Number(number) => self.generate_expr_number(number),
            NodeExpr::String(string) => self.generate_string(string),
        }
    }

    fn generate_expr_number(&self, number: &NodeExprNumber) -> String {
        number.value.to_string()
    }
    
}
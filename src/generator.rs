use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::parser::{Node, NodeExpr, NodeExprIdent, NodeExprLen, NodeExprNumber, NodeExprString, NodeFunc, NodeStmt, NodeStmtDefine, NodeStmtIf, NodeStmtWhile};

static LABEL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Generator {
    node: Node,
}

impl Generator {
    pub fn new(node: Node) -> Generator {
        Generator {
            node,
        }
    }

    fn unique_label(&self) -> String { 
        let count = LABEL_COUNT.fetch_add(1, Ordering::SeqCst);
        format!("label_{}", count)
    }

    fn generate_if_statement(&self, if_stmt: &NodeStmtIf) -> String {
        let mut result = String::new();
        let unique_label = self.unique_label();
        match &if_stmt.condition {
            NodeExpr::Equal(equal_expr) => {
                let left = self.generate_expr(&equal_expr.left);
                let right = self.generate_expr(&equal_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  jne .if_true_{}\n", unique_label));
            }
            NodeExpr::Lesser(lesser_expr) => {
                let left = self.generate_expr(&lesser_expr.left);
                let right = self.generate_expr(&lesser_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  jge .if_true_{}\n", unique_label));
            }
            NodeExpr::Greater(greater_expr) => {
                let left = self.generate_expr(&greater_expr.left);
                let right = self.generate_expr(&greater_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  jle .if_true_{}\n", unique_label));
            }
            NodeExpr::NotEqual(not_equal_expr) => {
                let left = self.generate_expr(&not_equal_expr.left);
                let right = self.generate_expr(&not_equal_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  je .if_true_{}\n", unique_label));
            }
            _ => panic!("Unsupported if statement condition"),
        }

        for stmt in &if_stmt.body {
            result.push_str(&self.generate_statement(stmt));
        }

        result.push_str(&format!(".if_true_{}:\n", unique_label));

        result
    }


    fn generate_while(&self, while_stmt: &NodeStmtWhile) -> String {
        let mut result = String::new();
        let unique_label = self.unique_label();
        result.push_str(&format!(".while_{}:\n", unique_label));

        match &while_stmt.condition {
            NodeExpr::Equal(equal_expr) => {
                let left = self.generate_expr(&equal_expr.left);
                let right = self.generate_expr(&equal_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  jne .while_end_{}\n", unique_label));
            }
            NodeExpr::Lesser(lesser_expr) => {
                let left = self.generate_expr(&lesser_expr.left);
                let right = self.generate_expr(&lesser_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  jge .while_end_{}\n", unique_label));
            }
            NodeExpr::Greater(greater_expr) => {
                let left = self.generate_expr(&greater_expr.left);
                let right = self.generate_expr(&greater_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  jle .while_end_{}\n", unique_label));
            }
            NodeExpr::NotEqual(not_equal_expr) => {
                let left = self.generate_expr(&not_equal_expr.left);
                let right = self.generate_expr(&not_equal_expr.right);
                result.push_str(&format!("  cmp {}, {}\n", left, right));
                result.push_str(&format!("  je .while_end_{}\n", unique_label));
            }
            _ => panic!("Unsupported while statement condition"),
        }

        for stmt in &while_stmt.body {
            result.push_str(&self.generate_statement(stmt));
        }

        result.push_str(&format!("  jmp .while_{}\n", unique_label));
        result.push_str(&format!(".while_end_{}:\n", unique_label));

        result
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
                let newstring = self.string_to_hex(self.generate_expr(&assign.expr));
                let mut result = String::new();
                result.push_str(&format!("  {} db {}\n", self.generate_expr_ident(&assign.ident), newstring));

                result.push_str(&format!("  {}_len equ $ - {}\n", self.generate_expr_ident(&assign.ident), self.generate_expr_ident(&assign.ident)));

                result
            }
            NodeStmt::If(if_stmt) => {
                self.generate_if_statement(if_stmt, )
            }
            NodeStmt::While(while_stmt) => {
                self.generate_while(while_stmt)
            }
            NodeStmt::Push(push) => {
                format!("  push {}\n", self.generate_expr(&push.expr))
            }
            NodeStmt::Pop(pop) => {
                format!("  pop {}\n", self.generate_expr_ident(&pop.ident))
            }
            NodeStmt::Xor(xor) => {
                format!("  xor {}, {}\n", self.generate_expr_ident(&xor.ident), self.generate_expr(&xor.expr))
            }
            _ => "".to_string(),
        }
    }

    fn string_to_hex(&self, string: String) -> String {
        let bytes = string.as_bytes();
        let mut hex_representation = String::new();
        let mut skip_next = false;

        for (i, &byte) in bytes.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            if byte == b'\\' && i + 1 < bytes.len() && bytes[i + 1] == b'n' {
                hex_representation.push_str("0x0a, ");
                skip_next = true; 
            } else {
                let hex = format!("0x{:02x}, ", byte);
                hex_representation.push_str(&hex);
            }
        }

        hex_representation.trim_end_matches(", ").to_string()
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
            _ => "".to_string(),
        }
    }

    fn generate_define(&self, define: &NodeStmtDefine) -> String {
        format!("%define {} {}\n", self.generate_expr_ident(&define.ident), self.generate_expr(&define.expr))
    }


    pub fn generate(&self) -> String {
        let mut result = String::new();

        for define in &self.node.defines {
            result.push_str(&self.generate_define(define));
        }

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
        format!("{}", string.value)
    }

    fn generate_length(&self, string: &NodeExprLen) -> String {
        format!("{}_len", self.generate_expr(&string.ident))
    }

    fn generate_expr(&self, expr: &NodeExpr) -> String {
        match expr {
            NodeExpr::Ident(ident) => self.generate_expr_ident(ident),
            NodeExpr::Number(number) => self.generate_expr_number(number),
            NodeExpr::String(string) => self.generate_string(string),
            NodeExpr::Len(string) => self.generate_length(string),
            _ => "".to_string(),
        }
    }

    fn generate_expr_number(&self, number: &NodeExprNumber) -> String {
        number.value.to_string()
    }
    
}
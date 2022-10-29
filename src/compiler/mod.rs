#![allow(dead_code)]

use std::{fs::File, io::Write};

use crate::parse::{
    ast::{Expr, Statement, StatementList},
    token::TokenType,
};

#[derive(Debug)]
pub struct Compiler {
    ast: StatementList,
    file: File,
}

impl Compiler {
    pub fn new(ast: StatementList) -> Self {
        let mut file = File::create("out.asm").expect("Cannot create asm file.");
        file.write_all(
            b"global .text
    global _start
_start:
        ",
        )
        .expect("Could not append basic assembly structure such as main entry point");
        return Compiler { ast, file };
    }

    pub fn compile(&mut self) {
        let ast = self.ast.clone();
        for statement in ast.0 {
            self.eval(statement);
        }
    }

    fn eval(&mut self, statement: Statement) {
        /* match statement {
            Statement::Expr(expr) => match expr {
                Expr::ExprStatement(token) => match token.r#type {
                    TokenType::String => {
                        self.file.write(b"STRING");
                    }
                    TokenType::Integers => {
                        self.file.write(b"NATURAL");
                    }
                    _ => panic!("Unimplemented"),
                },
                Expr::BinaryExpr { op, left, right } => match op {},
            },
            Statement::Block(s_list) => {
                for s in s_list {
                    &self.eval(s);
                }
            }
        } */
    }
}

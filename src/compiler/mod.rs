use std::{fs::File, io::Write};

use crate::parse::{Statement, StatementList};

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

    pub fn compile(&self) {
        for statement in &self.ast {
            &self.eval(statement);
        }
    }

    fn eval(&self, statement: &Statement) {
        match statement {
            Statement::Expr(token) => match token.r#type {
                _ => panic!("Unimplemented"),
            },
            Statement::Block(s_list) => {
                for s in s_list {
                    &self.eval(s);
                }
            }
        }
    }
}

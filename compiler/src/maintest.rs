use codegen::{Block, Function, Scope};

use codegen::{Field, Function, Scope};
use lexer::{Expression, Program, Statement, Variable};

pub struct RustCompiler {
    program: Program,
}

impl RustCompiler {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn compile(&self) {
        let Program {
            globals,
            imports,
            func,
        } = &self.program;

        let mut scope = Scope::new();

        for statement in globals.iter() {
            match statement {
                Statement::Declare(Variable { name, .. }, Some(expr)) => {
                    scope.raw(format!("const {} = {:#?};", name, expr).as_ref());
                    // won't workles try with normal one first I mean rust does not check code /shrug
                }
                _ => {}
            }
        }
        println!("{}", scope.to_string())
    }
}

fn main() {
    let mut scope = Scope::new();
    scope
        .new_struct("Foo")
        .derive("Debug")
        .field("one", "usize")
        .field("two", "String");

    // scope.new_module("std").scope().new_module("fs").new_fn();

    scope
        .new_fn("add")
        .vis("pub")
        .arg("a", "i32")
        .arg("b", "i32")
        .ret("i32")
        .line("a + b");

    println!("{}", scope.to_string());
}
/*
#[derive(Debug)]
struct Foo {
    one: usize,
    two: String,
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
*/

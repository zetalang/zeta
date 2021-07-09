extern crate gccjit;

use std::mem;

use gccjit::{Block, Context, Function as gFunc, FunctionType, ToRValue, Type};
use lexer::{Function, Program, Statement, Variable};
pub fn compile(context: Context<'_>, lexer: Program) {
    let Program {
        imports,
        func,
        globals,
    } = lexer;
    compile_fn(func, &context)
}
pub fn compile_fn(funcs: Vec<Function>, context: &Context<'_>) {
    let int_ty = context.new_type::<i64>();

    let bool_ty = context.new_type::<bool>();
    let void_ty = context.new_type::<()>();

    for func in funcs.iter() {
        let Function {
            is_async,
            name,
            return_type,
            arguments,
            statements,
        } = func;
        let r = match return_type {
            lexer::Type::Bool => bool_ty,
            lexer::Type::Str => todo!(),
            lexer::Type::Void => void_ty,
            lexer::Type::Int => int_ty,
            lexer::Type::Mlstr => todo!(),
            lexer::Type::Char => todo!(),
        };
        let fun = context.new_function(None, FunctionType::Exported, r, &[], name, false);
        let block = fun.new_block("entry");
        compile_statemnet(&block, statements, &fun)
    }
    let result = context.compile();
    let func = result.get_function(funcs[0].name.clone());
    let jit_compiled_fun: extern "C" fn() = if !func.is_null() {
        unsafe { mem::transmute(func) }
    } else {
        panic!("failed to retrieve function")
    };
    println!("the square of 2 is: {:#?}", jit_compiled_fun());
    // println!("the square of 10 is: {}", jit_compiled_fun(10));
    // println!("the square of -2 is: {}", jit_compiled_fun(-2));
}

pub fn compile_statemnet(block: &Block, statements: &Vec<Statement>, fun: &gFunc) {
    block.end_with_void_return(None);
    for statement in statements.iter() {
        match statement {
            Statement::Declare(a, b) => {}
            Statement::Return(_) => todo!(),
            Statement::If(_, _, _) => todo!(),
            Statement::While(_, _) => todo!(),
            Statement::Exp(_) => todo!(),
            Statement::Compound(_) => todo!(),
        }
    }
}

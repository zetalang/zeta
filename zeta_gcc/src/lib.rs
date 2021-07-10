extern crate gccjit;

use std::mem;

use gccjit::{
    Block, Context, Function as gFunc, FunctionType, Parameter, ToLValue, ToRValue, Type,
};
use lexer::{Function, Program, Statement, Variable};
const MEMORY_SIZE: i32 = 1000;

pub struct InitData<'a> {
    getchar: gFunc<'a>,
    parameter: Parameter<'a>,
    putchar: gFunc<'a>,
    memory_ty: Type<'a>,
    char_ptr: Type<'a>,
    void_param: Parameter<'a>,
    size_t_param: Parameter<'a>,
    void_ptr_ty: Type<'a>,
    memset: gFunc<'a>,
}
pub fn compile(context: Context<'_>, lexer: Program) {
    let Program {
        imports,
        func,
        globals,
    } = lexer;
    compile_fn(func, &context)
}
fn types<'a>(context: &'a Context<'_>) -> (Type<'a>, Type<'a>, Type<'a>, Type<'a>) {
    let int_ty = context.new_type::<i64>();

    let char_ty = context.new_type::<u8>();
    let bool_ty = context.new_type::<bool>();
    let void_ty = context.new_type::<()>();
    (int_ty, bool_ty, void_ty, char_ty)
}
pub fn init<'a>(context: &'a gccjit::Context<'a>) -> InitData<'a> {
    let (int_ty, bool_ty, void_ty, char_ty) = types(context);
    let getchar = context.new_function(
        None,
        gccjit::FunctionType::Extern,
        char_ty,
        &[],
        "getchar",
        false,
    );
    let parameter = context.new_parameter(None, char_ty, "c");
    let putchar = context.new_function(
        None,
        gccjit::FunctionType::Extern,
        void_ty,
        &[parameter],
        "putchar",
        false,
    );
    let memory_ty = context.new_array_type(None, char_ty, MEMORY_SIZE);
    let char_ptr = context.new_type::<u8>().make_pointer();
    let void_param = context.new_parameter(None, char_ptr, "ptr");
    let size_t_param = context.new_parameter(None, int_ty, "size");
    let int_param = context.new_parameter(None, int_ty, "num");
    let void_ptr_ty = context.new_type::<*mut ()>();
    let memset = context.new_function(
        None,
        gccjit::FunctionType::Extern,
        void_ptr_ty,
        &[void_param, int_param, size_t_param],
        "memset",
        false,
    );
    InitData {
        getchar,
        parameter,
        putchar,
        memory_ty,
        char_ptr,
        void_param,
        size_t_param,
        void_ptr_ty,
        memset,
    }
}
pub fn compile_fn(funcs: Vec<Function>, context: &Context<'_>) {
    let (int_ty, bool_ty, void_ty, char_ty) = types(context);
    let init = init(context);
    let f_main = context.new_function(
        None,
        gccjit::FunctionType::Exported,
        void_ty,
        &[],
        "main",
        false,
    );
    let size = context.new_rvalue_from_int(int_ty, MEMORY_SIZE);
    let array = f_main.new_local(None, init.memory_ty, "memory");
    let memory_ptr = f_main.new_local(None, int_ty, "memory_ptr");
    let mut current_block = f_main.new_block("entry_block");
    let zero_access =
        context.new_array_access(None, array.to_rvalue(), context.new_rvalue_zero(int_ty));
    current_block.add_eval(
        None,
        context.new_call(
            None,
            init.memset,
            &[
                zero_access.get_address(None),
                context.new_rvalue_zero(int_ty),
                size,
            ],
        ),
    );

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
        compile_statement(&block, statements, &fun, &init, context);
        block.end_with_void_return(None)
    }
    current_block.end_with_void_return(None);
    let result = context.compile();
    let func = result.get_function(funcs[0].name.clone());
    let jit_compiled_fun: extern "C" fn() = if !func.is_null() {
        unsafe { mem::transmute(func) }
    } else {
        panic!("failed to retrieve function")
    };
    println!("the square of 2 is: {:#?}", jit_compiled_fun());
    context.compile_to_file(gccjit::OutputKind::Executable, "main")
    // println!("the square of 10 is: {}", jit_compiled_fun(10));
    // println!("the square of -2 is: {}", jit_compiled_fun(-2));
}

pub fn compile_statement(
    block: &Block,
    statements: &Vec<Statement>,
    fun: &gFunc,
    init: &InitData,
    context: &Context<'_>,
) {
    let (int_ty, bool_ty, void_ty, char_ty) = types(context);
    let size = context.new_rvalue_from_int(int_ty, MEMORY_SIZE);
    let array = fun.new_local(None, init.memory_ty, "memory");
    let memory_ptr = fun.new_local(None, int_ty, "memory_ptr");
    let zero_access =
        context.new_array_access(None, array.to_rvalue(), context.new_rvalue_zero(int_ty));
    block.add_eval(
        None,
        context.new_call(
            None,
            init.memset,
            &[
                zero_access.get_address(None),
                context.new_rvalue_zero(int_ty),
                size,
            ],
        ),
    );
    for statement in statements.iter() {
        match statement {
            Statement::Declare(a, b) => {
                let access =
                    context.new_array_access(None, array.to_rvalue(), memory_ptr.to_rvalue());
                block.add_assignment_op(
                    None,
                    access,
                    gccjit::BinaryOp::Plus,
                    context.new_rvalue_one(char_ty),
                );
            }
            Statement::Return(_) => todo!(),
            Statement::If(_, _, _) => todo!(),
            Statement::While(_, _) => todo!(),
            Statement::Exp(_) => todo!(),
            Statement::Compound(_) => todo!(),
        }
    }
}

extern crate gccjit;

use gccjit::{
    Block, Context, Function as gFunc, FunctionType, OptimizationLevel, Parameter, ToLValue,
    ToRValue, Type,
};
use lexer::{BinOp, Expression, Function, Program, Statement, Variable};
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
pub struct Compile<'a> {
    context: gccjit::Context<'a>,
}

impl Compile<'static> {
    pub fn new<'a>() -> Self {
        let context = Context::default();
        context.set_dump_code_on_compile(true);
        context.set_optimization_level(OptimizationLevel::Aggressive);
        Self { context }
    }
    pub fn compile<'a>(&self, lexer: Program) {
        let Program {
            imports,
            func,
            globals,
        } = lexer;
        self.compile_fn(func)
    }
    fn types<'a>(&'a self) -> (Type<'a>, Type<'a>, Type<'a>, Type<'a>) {
        let int_ty: Type<'a> = self.context.new_type::<i64>();

        let char_ty: Type<'a> = self.context.new_type::<u8>();
        let bool_ty: Type<'a> = self.context.new_type::<bool>();
        let void_ty: Type<'a> = self.context.new_type::<()>();
        (int_ty, bool_ty, void_ty, char_ty)
    }
    pub fn init<'a>(&'a self) -> InitData<'a> {
        let (int_ty, bool_ty, void_ty, char_ty) = self.types();
        let getchar = self.context.new_function(
            None,
            gccjit::FunctionType::Extern,
            char_ty,
            &[],
            "getchar",
            false,
        );
        let parameter = self.context.new_parameter(None, char_ty, "c");
        let putchar = self.context.new_function(
            None,
            gccjit::FunctionType::Extern,
            void_ty,
            &[parameter],
            "putchar",
            false,
        );
        let memory_ty = self.context.new_array_type(None, char_ty, MEMORY_SIZE);
        let char_ptr = self.context.new_type::<u8>().make_pointer();
        let void_param = self.context.new_parameter(None, char_ptr, "ptr");
        let size_t_param = self.context.new_parameter(None, int_ty, "size");
        let int_param = self.context.new_parameter(None, int_ty, "num");
        let void_ptr_ty = self.context.new_type::<*mut ()>();
        let memset = self.context.new_function(
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
    pub fn compile_fn<'a>(&'a self, funcs: Vec<Function>) {
        let (int_ty, bool_ty, void_ty, char_ty) = self.types();
        let initialize = self.init();
        let f_main = self.context.new_function(
            None,
            gccjit::FunctionType::Exported,
            void_ty,
            &[],
            "main",
            false,
        );
        let size = self.context.new_rvalue_from_int(int_ty, MEMORY_SIZE);
        let array = f_main.new_local(None, initialize.memory_ty, "memory");
        let current_block = f_main.new_block("entry_block");
        let zero_access = self.context.new_array_access(
            None,
            array.to_rvalue(),
            self.context.new_rvalue_zero(int_ty),
        );
        current_block.add_eval(
            None,
            self.context.new_call(
                None,
                initialize.memset,
                &[
                    zero_access.get_address(None),
                    self.context.new_rvalue_zero(int_ty),
                    size,
                ],
            ),
        );

        current_block.end_with_void_return(None);
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

            let parameter = self.context.new_parameter(None, int_ty, "x");
            let fun = self.context.new_function(
                None,
                FunctionType::Exported,
                r,
                &[parameter],
                name,
                false,
            );
            let initi = self.init();
            let block = fun.new_block("entry");
            self.compile_statement(&block, statements, &fun, &initi);
            block.end_with_void_return(None)
        }
        self.context
            .compile_to_file(gccjit::OutputKind::Executable, "main")
    }

    pub fn compile_statement<'a>(
        &'a self,
        block: &Block,
        statements: &Vec<Statement>,
        fun: &gFunc,
        init: &InitData,
    ) {
        let (int_ty, bool_ty, void_ty, char_ty) = self.types();
        let size = self.context.new_rvalue_from_int(int_ty, MEMORY_SIZE);
        let array = fun.new_local(None, init.memory_ty, "memory");
        let memory_ptr = fun.new_local(None, int_ty, "memory_ptr");
        let zero_access = self.context.new_array_access(
            None,
            array.to_rvalue(),
            self.context.new_rvalue_zero(int_ty),
        );
        block.add_eval(
            None,
            self.context.new_call(
                None,
                init.memset,
                &[
                    zero_access.get_address(None),
                    self.context.new_rvalue_zero(int_ty),
                    size,
                ],
            ),
        );
        for statement in statements.iter() {
            match statement {
                Statement::Declare(a, b) => {
                    let expr = self.compile_exp(b, block, fun);
                }
                Statement::Return(a) => todo!(),
                Statement::If(_, _, _) => todo!(),
                Statement::While(_, _) => todo!(),
                Statement::Exp(_) => todo!(),
                Statement::Compound(_) => todo!(),
            }
        }
    }
    fn compile_binop<'a>(
        &self,
        binop: &BinOp,
    ) -> (Option<gccjit::BinaryOp>, Option<gccjit::ComparisonOp>) {
        let mut bnop = None;
        let mut cop = None;
        match binop {
            BinOp::Addition => bnop = Some(gccjit::BinaryOp::Plus),
            BinOp::Subtraction => bnop = Some(gccjit::BinaryOp::Minus),
            BinOp::Multiplication => bnop = Some(gccjit::BinaryOp::Mult),
            BinOp::Division => bnop = Some(gccjit::BinaryOp::Divide),
            BinOp::Modulus => bnop = Some(gccjit::BinaryOp::Modulo),
            BinOp::LessThan => cop = Some(gccjit::ComparisonOp::LessThan),
            BinOp::LessThanOrEqual => cop = Some(gccjit::ComparisonOp::LessThanEquals),
            BinOp::GreaterThan => cop = Some(gccjit::ComparisonOp::GreaterThan),
            BinOp::GreaterThanOrEqual => cop = Some(gccjit::ComparisonOp::GreaterThanEquals),
            BinOp::Equal => cop = Some(gccjit::ComparisonOp::Equals),
            BinOp::NotEqual => cop = Some(gccjit::ComparisonOp::NotEquals),
            BinOp::And => bnop = Some(gccjit::BinaryOp::LogicalAnd),
            BinOp::Or => bnop = Some(gccjit::BinaryOp::LogicalOr),
            BinOp::BitwiseLeft => bnop = Some(gccjit::BinaryOp::LShift),
            BinOp::BitwiseRight => bnop = Some(gccjit::BinaryOp::RShift),
            BinOp::BitwiseAnd => bnop = Some(gccjit::BinaryOp::BitwiseAnd),
            BinOp::BitwiseXor => bnop = Some(gccjit::BinaryOp::BitwiseXor),
            BinOp::BitwiseOr => bnop = Some(gccjit::BinaryOp::BitwiseOr),
            BinOp::Comma => bnop = Some(gccjit::BinaryOp::Comma),
        };
        (bnop, cop)
    }
    fn compile_exp<'a>(&self, expr: &Option<Expression>, block: &Block, fun: &gFunc) {
        let (int_ty, bool_ty, void_ty, char_ty) = self.types();

        match expr {
            Some(exp) => match exp {
                Expression::BinOp(a, b, c) => {
                    let binop = self.compile_binop(a);
                    let parm = fun.get_param(0).to_rvalue();
                    let loc = fun.new_local(None, int_ty, "abc");
                    println!("{:#?}", loc);
                    match binop.0 {
                        Some(e) => block.add_assignment_op(None, loc, e, parm),
                        None => todo!(),
                    }
                }
                Expression::UnOp(_, _) => todo!(),
                Expression::Int(_) => todo!(),
                Expression::Char(_) => todo!(),
                Expression::MLStr(_) => todo!(),
                Expression::FunctionCall(_, _) => todo!(),
                Expression::Bool(_) => todo!(),
                Expression::Variable(_) => todo!(),
                Expression::VariableRef(_) => todo!(),
                Expression::Assign(_, _) => todo!(),
                Expression::AssignPostfix(_, _) => todo!(),
                Expression::Ternary(_, _, _) => todo!(),
            },
            None => todo!(),
        }
    }
}

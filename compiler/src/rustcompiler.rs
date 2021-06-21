use codegen::{Function as CodegenFunc, Scope};
use lexer::{Expression, Program, Statement, Type, Variable};

pub trait Compiler {
    fn new(program: Program) -> Self;
    fn compile(&self) -> String;
}

pub struct RustCompiler {
    program: Program,
}

impl RustCompiler {
    fn compile_expr(&self, exp: &Expression, var_type: &str) -> String {
        match exp {
            Expression::FunctionCall(varname, fnargs) => {
                let mut s = String::new();
                for i in 0..fnargs.len() {
                    match &fnargs[i] {
                        Expression::Variable(name) => {
                            if i == 0 {
                                s = s + "" + &name;
                            } else {
                                s = s + "," + &name;
                            }
                        }
                        _ => unimplemented!(),
                    };
                }
                format!("{}({})", varname, s)
            }
            Expression::Variable(n) => {
                if var_type == "str" {
                    format!("\"{}\"", n)
                } else {
                    format!("{}", n)
                }
            }
            Expression::Int(num) => {
                format!("{}", num)
            }
            _ => unimplemented!(),
        }
    }

    fn compile_statement(&self, statement: &Statement) -> String {
        match statement {
            Statement::Declare(var, Some(exp)) => {
                format!("let {} = {};", var.name, self.compile_expr(exp, &var.t))
            }
            _ => unimplemented!(),
        }
    }
}

impl Compiler for RustCompiler {
    fn new(program: Program) -> Self {
        Self { program }
    }

    fn compile(&self) -> String {
        let Program {
            globals,
            imports,
            func,
        } = &self.program;

        let mut scope = Scope::new();

        for statement in globals.iter() {
            match statement {
                Statement::Declare(Variable { name, .. }, Some(expr)) => {
                    scope.raw(
                        format!(
                            "const {}: &str = {:#?};", /* TODO don't hardcode &str */
                            name,
                            match expr {
                                Expression::Variable(value) => {
                                    value
                                }
                                _ => unimplemented!(),
                            }
                        )
                        .as_ref(),
                    );
                }
                _ => {}
            }
        }
        for function in func.iter() {
            let mut t = "";
            let mut f = CodegenFunc::new(&function.name);
            if function.return_type == Type::Bool {
                t = "bool";
            } else if function.return_type == Type::Char {
                t = "str";
            } else if function.return_type == Type::Int {
                t = "int";
            } else if function.return_type == Type::Mlstr {
                t = "str";
            } else if function.return_type == Type::Str {
                t = "str";
            }
            if t != "" {
                f.ret(t);
            }
            for arg in 0..function.arguments.len() {
                let a = &function.arguments[arg];
                if a.t == "str" {
                    f.arg(&a.name, "&".to_owned() + &function.arguments[arg].t);
                } else {
                    f.arg(&a.name, &function.arguments[arg].t);
                }
            }
            for i in function.statements.iter() {
                f.line(self.compile_statement(i));
            }
            f.set_async(function.is_async);
            scope.push_fn(f);
        }
        return scope.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::{tokenize, Parser};

    #[test]
    fn it_works() {
        let tokens = tokenize("const x = 23").unwrap();
        let mut parser = Parser::new(tokens);
        let compiler = RustCompiler::new(parser.parse().unwrap());
    }
}

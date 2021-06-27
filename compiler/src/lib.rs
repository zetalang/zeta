// pub mod compiler;
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
                for (i, _exp) in fnargs.iter().enumerate() {
                    match &fnargs[i] {
                        Expression::Variable(name) => {
                            if i == 0 {
                                s = s + "" + &name;
                            } else {
                                s = s + "," + &name;
                            }
                        }
                        Expression::Bool(name) => {
                            if i == 0 {
                                s = s + "" + &name.to_string();
                            } else {
                                s = s + "," + &name.to_string();
                            }
                        }
                        Expression::MLStr(name) => {
                            if i == 0 {
                                s = "\"".to_owned() + &s + "" + &name + "\"";
                            } else {
                                s =     "\"".to_owned() + &s + "," + &name + "\"";
                            }
                        }
                        Expression::Char(name) => {
                            if i == 0 {
                                s = "\"".to_owned() + &s + "" + &name + "\"";
                            } else {
                                s =     "\"".to_owned() + &s + "," + &name + "\"";
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
                    n.to_string()
                }
            }
            Expression::Bool(b) => {
                format!("{}", b)
            }
            Expression::Int(num) => {
                format!("{}", num)
            }
            Expression::MLStr(name) => {
                    "\"".to_owned() + &name + "\""
            }
            Expression::Char(name) => {
                    "\"".to_owned() + &name + "\""
            }

            other => unimplemented!(),
        }
    }

    fn compile_statement(&self, statement: &Statement) -> String {
        match statement {
            Statement::Declare(var, Some(exp)) => {
                // panic!("{:#?}", exp);
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
            if let Statement::Declare(Variable { name, .. }, Some(expr)) = statement {
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
            } else if function.return_type == Type::Mlstr || function.return_type == Type::Str {
                t = "str";
            }
            if !t.is_empty() {
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
        scope.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::{tokenize, Parser};

    #[test]
    fn it_works() {
        let tokens = tokenize("const x = 23").unwrap();
        let mut parser = Parser::new(tokens, "".into());
        let token = parser.parse().unwrap();
        let compiler = RustCompiler::new(token.unwrap().0);
    }
}

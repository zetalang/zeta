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
                    scope.raw(
                        format!(
                            "const {} = {:#?};",
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
        println!("{}", scope.to_string())
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

pub mod rustcompiler;
use lexer::Program;
pub trait Compiler {
    fn new(program: Program) -> Self;
    fn compile(&self) -> String;
}

#[cfg(test)]
mod tests {
    use crate::rustcompiler::RustCompiler;

    use super::*;
    use lexer::{tokenize, Parser};

    #[test]
    fn it_works() {
        let tokens = tokenize("const x = 23", "").unwrap();
        let mut parser = Parser::new(tokens, "".into());
        let token = parser.parse().unwrap();
        let compiler = RustCompiler::new(token.unwrap().0);
    }
}

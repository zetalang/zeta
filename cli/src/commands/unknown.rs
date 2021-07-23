use super::Command;
use crate::{utils::App, utils::VERSION};
use anyhow::{Context as ic, Result};
use async_trait::async_trait;
use colored::Colorize;
use compiler::{rustcompiler::RustCompiler, Compiler};
use gccjit::{Context, FunctionType, ToRValue};
use lexer::{tokenize, Parser};
use no_comment::{languages, IntoWithoutComments as _};
use std::mem;
use std::{io::Read, sync::Arc, vec};
use zeta_gcc::Compile as zCompile;

use std::default::Default;
extern crate gccjit;

use gccjit::OptimizationLevel;
fn test() {
    let context = Context::default();
    context.set_dump_code_on_compile(true);
    context.set_optimization_level(OptimizationLevel::Aggressive);
    // context.compile_to_file(gccjit::OutputKind::ObjectFile, "a.out");
    let int_ty = context.new_type::<i32>();
    let parameter = context.new_parameter(None, int_ty, "x");
    let fun = context.new_function(
        None,
        FunctionType::Exported,
        int_ty,
        &[parameter],
        "main",
        false,
    );
    let block = fun.new_block("main_block");
    let parm = fun.get_param(0).to_rvalue();
    let square = parm * parm;
    block.end_with_return(None, square);
    let result = context.compile();
    // let func = result.get_function("main");
    // let jit_compiled_fun: extern "C" fn(i32) -> i32 = if !func.is_null() {
    //     unsafe { mem::transmute(func) }
    // } else {
    //     panic!("failed to retrieve function")
    // };
    // println!("the square of 2 is: {}", jit_compiled_fun(2));
    // println!("the square of 10 is: {}", jit_compiled_fun(10));
    // println!("the square of -2 is: {}", jit_compiled_fun(-2));
}
pub struct Compile;

#[async_trait]
impl Command for Compile {
    fn help() -> String {
        format!(
            r#"zetac {}
    
Usage: {} {} {}
Commands:
  {asterisk} {} - Compiles the given file
Flags: 
  {asterisk} {} - uses clang (default)
  {asterisk} {} - uses gcc
  {asterisk} {} - verbose output
  {asterisk} {} - Builds for deployement
  {asterisk} {} - Compiles to rust code
  "#,
            VERSION.bright_green().bold(),
            "zetac".bright_green().bold(),
            "[commands]".bright_purple(),
            "[flags]".bright_purple(),
            "<filename>".bright_blue(),
            "--useclang, -ucg".bright_blue(),
            "--usegcc, -ugcc ".bright_blue(),
            "--verbose, -vb  ".bright_blue(),
            "--release, -r   ".bright_blue(),
            "--userust       ".bright_blue(),
            asterisk = "*".bright_magenta().bold(),
        )
    }

    async fn exec(app: Arc<App>) -> Result<()> {
        let acceptedflags: Vec<&str> = vec![
            "--useclang",
            "-ucg",
            "--usegcc",
            "-ugcc",
            "--verbose",
            "-vb",
            "--release",
            "-r",
            "--userust",
        ];
        let _flags = app.filter_flag(&acceptedflags);
        let args = app.args.clone();
        let filename: &str = args[0].as_str();
        let mut file =
            std::fs::File::open(filename).unwrap_or_else(|e| app.error(e.to_string().as_str()));
        let mut f_contents = String::new();

        file.read_to_string(&mut f_contents)
            .unwrap_or_else(|e| app.error(e.to_string().as_str()));

        let preprocessed = f_contents
            .chars()
            .without_comments(languages::rust())
            .collect::<String>();
        let tokenize = tokenize(&preprocessed, &filename)
            .context("Failed to tokenize the file contents.".red().bold())?;
        let mut parse = Parser::new(tokenize, filename.into());
        let parsedval = parse
            .parse()
            .context("ParserError: Failed to parse the contents".red().bold())?;
        let p1 = parsedval.clone().unwrap().0;
        // println!("{:#?}", parsedval);
        if app.has_flag(&["--userust"]) {
            let rustcompiler = RustCompiler::new(parsedval.unwrap().0);
            println!("{}", rustcompiler.compile());
        } else {
            println!("{:#?}", parsedval.unwrap().0);
            let gcc = zCompile::new();
            gcc.compile(p1);

            // zeta_gcc::compile(context, p1)
            // test()
        }
        Ok(())
    }
}

use std::collections::HashMap;
use llvm_sys::prelude::LLVMValueRef;

use torq_llvm::core::{self, Function, FunctionRef};
use torq_llvm::core::types::{RealTypeCtor, RealTypeRef};
use torq_llvm::{LLVMRef, LLVMRefCtor};
pub trait ModuleProvider {
    fn dump(&self);
    fn get_module(&mut self) -> &mut core::Module;
    fn get_function(&mut self, name: &str) -> Option<(FunctionRef, bool)>;
}

pub struct SimpleModuleProvider {
    module: core::Module
}

impl SimpleModuleProvider {
    pub fn new(name: &str) -> SimpleModuleProvider {
        let module = core::Module::new(name);

        SimpleModuleProvider {
            module: module
        }
    }
}

impl ModuleProvider for SimpleModuleProvider {
    fn dump(&self) {
        self.module.dump();
    }

    fn get_module(&mut self) -> &mut core::Module {
        &mut self.module
    }

    fn get_function(&mut self, name: &str) -> Option<(FunctionRef, bool)> {
        match self.module.get_function_by_name(name) {
            Some(f) => Some((f, f.count_basic_blocks() > 0)),
            None => None
        }
    }
}

pub struct Context {
    context: core::Context,
    builder: core::Builder,
    named_values: HashMap<String, LLVMValueRef>,
    ty: RealTypeRef
}

impl Context {
    pub fn new() -> Context {

        let context = core::Context::get_global();
        let builder = core::Builder::new();
        let named_values = HashMap::new();
        let ty = RealTypeRef::get_double();

        Context { context: context,
                  builder: builder,
                  named_values: named_values,
                  ty: ty
        }
    }
}
pub type IRBuildingResult = Result<(LLVMValueRef, bool), String>;

fn error(message : &str) -> IRBuildingResult {
    Err(message.to_string())
}

// pub trait IRBuilder {
//     fn codegen(&self, context: &mut Context, module_provider: &mut ModuleProvider) -> IRBuildingResult;
// }
// impl IRBuilder for parser::ParsingResult {
//     fn codegen(&self, context: &mut Context, module_provider: &mut ModuleProvider) -> IRBuildingResult {
//         match self {
//             &Ok((ref ast, _)) => ast.codegen(context, module_provider),
//             &Err(ref message) => Err(message.clone())
//         }
//     }
// }

// impl IRBuilder for Vec<parser::ASTNode> {
//     fn codegen(&self, context: &mut Context, module_provider: &mut ModuleProvider) -> IRBuildingResult {
//         let mut result = error("empty AST");
//         for node in self.iter() {
//             result = Ok(try!(node.codegen(context, module_provider)));
//         }

//         result
//     }
// }

// impl IRBuilder for parser::ASTNode {
//     fn codegen(&self, context: &mut Context, module_provider: &mut ModuleProvider) -> IRBuildingResult {
//         match self {
//             &parser::ExternNode(ref prototype) => prototype.codegen(context, module_provider),
//             &parser::FunctionNode(ref function) => function.codegen(context, module_provider)
//         }
//     }
// }

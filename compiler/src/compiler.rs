use std::collections::HashMap;
use llvm_sys::prelude::LLVMValueRef;

use torq_llvm::core::{self, Function, FunctionRef};
use torq_llvm::core::types::{RealTypeCtor, RealTypeRef};
use torq_llvm::{LLVMRef, LLVMRefCtor};

use lexer::ops::ParsingResult;

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

pub trait ModuleProvider {
    fn dump(&self);
    fn get_module(&mut self) -> &mut core::Module;
    fn get_function(&mut self, name: &str) -> Option<(FunctionRef, bool)>;
    fn get_pass_manager(&mut self) -> &mut core::FunctionPassManager;
}

pub fn new_module(name: &str) -> (core::Module, core::FunctionPassManager) {
    let module = core::Module::new(name);
    let mut function_passmanager = core::FunctionPassManager::new(&module);
    function_passmanager.add_promote_memory_to_register_pass();
    function_passmanager.add_basic_alias_analysis_pass();
    function_passmanager.add_instruction_combining_pass();
    function_passmanager.add_reassociate_pass();
    function_passmanager.add_GVN_pass();
    function_passmanager.add_CFG_simplification_pass();
    function_passmanager.initialize();

    (module, function_passmanager)
}

pub struct SimpleModuleProvider {
    function_passmanager: core::FunctionPassManager,
    module: core::Module
}

impl SimpleModuleProvider {
    pub fn new(name: &str) -> SimpleModuleProvider {
        let (module, function_passmanager) = new_module(name);
        SimpleModuleProvider {
            function_passmanager: function_passmanager,
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

    fn get_pass_manager(&mut self) -> &mut core::FunctionPassManager {
        &mut self.function_passmanager
    }
}
pub type IRBuildingResult = Result<(LLVMValueRef, bool), String>;

fn error(message : &str) -> IRBuildingResult {
    Err(message.to_string())
}

pub trait IRBuilder {
    fn codegen(&self, context: &mut Context, module_provider: &mut ModuleProvider) -> IRBuildingResult;
}
impl IRBuilder for ParsingResult {
    fn codegen(&self, context: &mut Context, module_provider: &mut ModuleProvider) -> IRBuildingResult {
        match self {
            &Ok((ref ast, _)) => self.codegen(context, module_provider),
            &Err(ref message) => Err(message.clone())
        }
    }
}

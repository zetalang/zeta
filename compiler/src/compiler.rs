use llvm_sys::{LLVMBuilder, LLVMContext, LLVMModule, LLVMValue};
use zeta_llvm::functions::CodegenFunc;

pub struct Compile {
    pub codegenfunc: CodegenFunc,
}

impl Compile {
    pub fn new(
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
    ) -> Self {
        Self {
            codegenfunc: CodegenFunc::new(context, module, builder),
        }
    }
    pub fn new_func(&self, name: String) -> u64 {
        self.codegenfunc.createFunc(&name)
    }
}

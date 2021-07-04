use std::mem;

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBuildAdd, LLVMBuildRet,
        LLVMContextDispose, LLVMDisposeBuilder, LLVMDumpModule, LLVMFunctionType, LLVMGetParam,
        LLVMInt64TypeInContext, LLVMPositionBuilderAtEnd,
    },
    execution_engine::{
        LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMGetFunctionAddress,
        LLVMLinkInMCJIT,
    },
    target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget},
    LLVMBuilder, LLVMContext, LLVMModule, LLVMValue,
};

pub struct CodegenFunc {
    pub context: *mut LLVMContext,
    pub module: *mut LLVMModule,
    pub builder: *mut LLVMBuilder,
}

impl CodegenFunc {
    pub fn new(
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
    ) -> Self {
        Self {
            context,
            builder,
            module,
        }
    }
    pub fn createFunc(&self, name: &str) -> u64 {
        let mut res = 2;
        unsafe {
            let i64t = LLVMInt64TypeInContext(self.context);
            let mut argts = [i64t, i64t, i64t];
            let function_type = LLVMFunctionType(i64t, argts.as_mut_ptr(), argts.len() as u32, 0);

            let function = LLVMAddFunction(self.module, name.as_ptr() as *const _, function_type);
            let bb = LLVMAppendBasicBlockInContext(
                self.context,
                function,
                b"entry\0".as_ptr() as *const _,
            );

            LLVMPositionBuilderAtEnd(self.builder, bb);

            let x = LLVMGetParam(function, 0);
            let y = LLVMGetParam(function, 1);
            let z = LLVMGetParam(function, 2);

            let sum = LLVMBuildAdd(self.builder, x, y, b"sum.1\0".as_ptr() as *const _);
            let sum = LLVMBuildAdd(self.builder, sum, z, b"sum.2\0".as_ptr() as *const _);

            // Emit a `ret void` into the function
            LLVMBuildRet(self.builder, sum);

            // done building
            LLVMDisposeBuilder(self.builder);

            // Dump the module as IR to stdout.
            let i = self.module;
            // LLVMDumpModule(self.module);

            // build an execution engine
            let mut ee = std::mem::MaybeUninit::zeroed().assume_init();
            let mut out = mem::zeroed();
            // robust code should check that these calls complete successfully
            // each of these calls is necessary to setup an execution engine which compiles to native
            // code
            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();

            // takes ownership of the module
            LLVMCreateExecutionEngineForModule(&mut ee, self.module, &mut out);

            let addr = LLVMGetFunctionAddress(ee, name.as_ptr() as *const _);

            let f: extern "C" fn(u64, u64, u64) -> u64 = mem::transmute(addr);

            let x: u64 = 2;
            let y: u64 = 2;
            let z: u64 = 1;
            res = f(x, y, z);
            println!("{}", res);
            LLVMDisposeExecutionEngine(ee);
            LLVMContextDispose(self.context);
        };
        // get the function's arguments
        res
    }
}

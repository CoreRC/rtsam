#[cfg(all(codegen,test))]
mod tests {
    // use super::*;

    use inkwell;
    use inkwell::builder::Builder;
    use inkwell::context::Context;
    use inkwell::execution_engine::{ExecutionEngine, JitFunction};
    use inkwell::module::Module;
    use inkwell::targets::Target;
    use inkwell::OptimizationLevel;
    use llvm_sys;

    /// Convenience type alias for the `sum` function.
    ///
    /// Calling this is innately `unsafe` because there's no guarantee it doesn't
    /// do `unsafe` operations internally.
    type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

    fn jit_compile_sum(
        context: &Context,
        module: &Module,
        builder: &Builder,
        execution_engine: &ExecutionEngine,
    ) -> Option<JitFunction<SumFunc>> {
        let i64_type = context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);

        let function = module.add_function("sum", fn_type, None);
        let basic_block = context.append_basic_block(&function, "entry");

        builder.position_at_end(&basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = builder.build_int_add(x, y, "sum");
        let sum = builder.build_int_add(sum, z, "sum");

        builder.build_return(Some(&sum));

        unsafe { execution_engine.get_function("sum").ok() }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct MyModule {
        pub(crate) non_global_context: Option<Context>, // REVIEW: Could we just set context to the global context?
        data_layout: std::cell::RefCell<Option<inkwell::data_layout::DataLayout>>,
        pub(crate) module: std::cell::Cell<llvm_sys::prelude::LLVMModuleRef>,
        pub(crate) owned_by_ee:
            std::cell::RefCell<Option<inkwell::execution_engine::ExecutionEngine>>,
    }

    #[test]
    fn inkwell_working() {
        println!("Testing LLVM codegen...");

        let context = Context::create();
        let module = context.create_module("sum");
        let builder = context.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();

        let sum = jit_compile_sum(&context, &module, &builder, &execution_engine)
            .ok_or("Unable to JIT compile `sum`")
            .unwrap();

        let x = 1u64;
        let y = 2u64;
        let z = 3u64;
        let result : u64;

        unsafe { result = sum.call(x, y, z); }

        println!("{} + {} + {} = {}", x, y, z, result);

        let mymod: MyModule = unsafe { std::mem::transmute(module) };

        println!("Emitted Assembly:");
        unsafe {
            llvm_sys::core::LLVMDumpModule(mymod.module.get());
        }

        assert_eq!(result, x + y + z);

    }

}

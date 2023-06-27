use pliron::context::Context;
use wasmparser::{FuncValidator, Operator, WasmModuleResources};

use crate::{func_builder::FuncBuilder, mod_builder::ModuleBuilder, WasmError};

/// Translates wasm operators into c2zk IR instructions.
#[allow(unused_variables)]
pub fn translate_operator(
    ctx: &mut Context,
    validator: &mut FuncValidator<impl WasmModuleResources>,
    op: &Operator,
    func_builder: &mut FuncBuilder,
    mod_builder: &mut ModuleBuilder,
) -> Result<(), WasmError> {
    match op {
        Operator::Unreachable => {
            func_builder.op().unreachable(ctx);
        }
        Operator::Nop => {
            func_builder.op().nop(ctx);
        }
        Operator::End => func_builder.op().end(ctx)?,
        Operator::Return => func_builder.op().ret(ctx)?,
        Operator::Call { function_index } => {
            let callee_name = mod_builder
                .get_func_name((*function_index).into())
                .ok_or_else(|| WasmError::User("Call index is not found".to_string()))?;
            func_builder.op().call(ctx, callee_name)?;
        }
        Operator::Loop { blockty } => {
            func_builder.op().bloop(ctx, blockty)?;
        }
        Operator::Block { blockty } => {
            func_builder.op().block(ctx, blockty)?;
        }
        Operator::BrIf { relative_depth } => {
            func_builder.op().br_if(ctx, *relative_depth);
        }
        Operator::Br { relative_depth } => {
            func_builder.op().br(ctx, *relative_depth);
        }
        Operator::LocalGet { local_index } => func_builder.op().local_get(ctx, *local_index),
        Operator::LocalTee { local_index } => func_builder.op().local_tee(ctx, *local_index),
        Operator::LocalSet { local_index } => func_builder.op().local_set(ctx, *local_index),
        Operator::I32Const { value } => func_builder.op().i32const(ctx, *value)?,
        Operator::I64Const { value } => func_builder.op().i64const(ctx, *value)?,
        Operator::I32Add => func_builder.op().i32add(ctx)?,
        Operator::I32Eqz => func_builder.op().i32eqz(ctx),
        Operator::I32WrapI64 => func_builder.op().i32wrapi64(ctx),
        Operator::I32GeU => func_builder.op().i32geu(ctx),
        Operator::I32And => func_builder.op().i32and(ctx),
        Operator::I64Add => func_builder.op().i64add(ctx),
        Operator::I64Eqz => func_builder.op().i64eqz(ctx),
        Operator::I64And => func_builder.op().i64and(ctx),
        Operator::I64GeU => func_builder.op().i64geu(ctx),
        Operator::I64Ne => func_builder.op().i64ne(ctx),
        Operator::I64Eq => func_builder.op().i64eq(ctx),
        Operator::I64ExtendI32U => func_builder.op().i64extendi32u(ctx),
        _ => todo!("Wasm op not implemented: {:?}", op),
    };
    Ok(())
}
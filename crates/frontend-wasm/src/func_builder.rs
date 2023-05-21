use ozk_wasm_dialect::ops::BlockOp;
use ozk_wasm_dialect::ops::FuncOp;
use ozk_wasm_dialect::ops::LoopOp;
use pliron::basic_block::BasicBlock;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::types::FunctionType;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::r#type::Type;
use pliron::r#type::TypeObj;
use thiserror::Error;

use crate::op_builder::OpBuilder;

pub struct FuncBuilder<'a> {
    ctx: &'a mut Context,
    name: String,
    sig: Option<FunctionType>,
    blocks: Vec<(Option<Ptr<Operation>>, Ptr<BasicBlock>)>,
    locals: Vec<Ptr<TypeObj>>,
}

impl<'a> FuncBuilder<'a> {
    pub fn new(ctx: &mut Context, name: String) -> FuncBuilder {
        FuncBuilder {
            name,
            sig: None,
            locals: Vec::new(),
            blocks: vec![(None, BasicBlock::new(ctx, None, Vec::new()))],
            ctx,
        }
    }

    pub fn declare_local(&mut self, count: u32, ty: Ptr<TypeObj>) {
        for _ in 0..count {
            self.locals.push(ty);
        }
    }

    pub fn declare_locals(&mut self, locals: Vec<Ptr<TypeObj>>) {
        self.locals.extend(locals);
    }

    pub fn build(self) -> Result<FuncOp, FuncBuilderError> {
        let sig = self.sig.ok_or_else(|| {
            FuncBuilderError::MissingSignature(format!("FuncBuilder for {}", self.name))
        })?;
        // TODO: should be a separate lowering pass
        // let mut locals_with_params = self.locals.clone();
        // let mut prepended_ins = self.ins.clone();
        // for (idx, param) in sig.params.iter().enumerate().rev() {
        //     locals_with_params.insert(0, *param);
        //     prepended_ins.insert(
        //         0,
        //         Inst::LocalSet {
        //             local_idx: idx as u32,
        //         },
        //     );
        // }
        Ok(FuncOp::new_unlinked(
            self.ctx,
            &self.name,
            Type::register_instance(sig, self.ctx),
        ))
    }

    pub fn op(&mut self) -> OpBuilder {
        OpBuilder::new(self.ctx, self)
    }

    pub fn push(&mut self, op: Ptr<Operation>) {
        let opop = &op.deref(self.ctx).get_op(self.ctx);
        if let Some(block) = opop.downcast_ref::<BlockOp>() {
            self.blocks
                .push((Some(block.get_operation()), block.get_block(&self.ctx)));
        } else if let Some(block) = opop.downcast_ref::<LoopOp>() {
            self.blocks
                .push((Some(block.get_operation()), block.get_block(&self.ctx)));
        } else {
            let current_bb = self.blocks.last_mut().unwrap().1;
            op.insert_at_back(current_bb, self.ctx);
        }
    }

    pub fn push_end(&mut self) {
        let (ending_block_op_opt, ending_bb) = self.blocks.pop().unwrap();
        let current_bb = self.blocks.last().unwrap().1;
        match ending_block_op_opt {
            Some(ending_block_op) => {
                ending_block_op.insert_at_back(current_bb, self.ctx);
            }
            None => {
                todo!("function end");
            }
        }
    }

    // pub fn push_insts(&mut self, insts: Vec<Inst>) {
    //     self.ins.extend(insts);
    // }

    pub fn set_signature(&mut self, signature: FunctionType) {
        self.sig = Some(signature);
    }

    pub fn set_name(&mut self, clone: String) {
        self.name = clone;
    }
}

#[derive(Debug, Error)]
pub enum FuncBuilderError {
    #[error("missing function signature")]
    MissingSignature(String),
}

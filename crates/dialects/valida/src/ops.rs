use intertrait::cast_to;
use ozk_ozk_dialect::attributes::FieldElemAttr;
use pliron::attribute;
use pliron::attribute::AttrObj;
use pliron::basic_block::BasicBlock;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::dialects::builtin::attributes::StringAttr;
use pliron::dialects::builtin::op_interfaces::OneRegionInterface;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::error::CompilerError;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;

declare_op!(
    /// Write the immediate values b,c,d,e to the cell located at offset a.
    Imm32Op,
    "imm32",
    "valida"
);

impl Imm32Op {
    /// Attribute key for operands.
    pub const ATTR_KEY_OPERAND_A: &str = "imm32.a";
    pub const ATTR_KEY_OPERAND_B: &str = "imm32.b";
    pub const ATTR_KEY_OPERAND_C: &str = "imm32.c";
    pub const ATTR_KEY_OPERAND_D: &str = "imm32.d";
    pub const ATTR_KEY_OPERAND_E: &str = "imm32.e";

    /// Create a new [ConstantOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(
        ctx: &mut Context,
        a: AttrObj,
        b: AttrObj,
        c: AttrObj,
        d: AttrObj,
        e: AttrObj,
    ) -> Imm32Op {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_A, a);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_B, b);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_C, c);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_D, d);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_E, e);
        Imm32Op { op }
    }

    // /// Get the constant value that this Op defines.
    // pub fn get_value(&self, ctx: &Context) -> AttrObj {
    //     let op = self.get_operation().deref(ctx);
    //     #[allow(clippy::expect_used)]
    //     let value = op
    //         .attributes
    //         .get(Self::ATTR_KEY_OPERANDS)
    //         .expect("no attribute found");
    //     if value.is::<VecAttr>() {
    //         todo!("extract ValidaOperands from VecAttr")
    //     } else {
    //         todo!("panic?")
    //     }
    // }

    fn get_operand(&self, ctx: &Context, operand_name: &str) -> AttrObj {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::panic)]
        let value = op.attributes.get(operand_name).unwrap_or_else(|| {
            panic!("no attribute for operand '{}' found", operand_name);
        });
        #[allow(clippy::panic)]
        if value.is::<FieldElemAttr>() {
            attribute::clone::<FieldElemAttr>(value)
        } else {
            panic!("expected FieldElemAttr, found {}", value.with_ctx(ctx));
        }
    }

    pub fn get_operand_a(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_A)
    }

    pub fn get_operand_b(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_B)
    }

    pub fn get_operand_c(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_C)
    }

    pub fn get_operand_d(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_D)
    }

    pub fn get_operand_e(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_E)
    }
}

impl DisplayWithContext for Imm32Op {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}(fp) {} {} {} {}",
            self.get_opid().with_ctx(ctx),
            self.get_operand_a(ctx).with_ctx(ctx),
            self.get_operand_b(ctx).with_ctx(ctx),
            self.get_operand_c(ctx).with_ctx(ctx),
            self.get_operand_d(ctx).with_ctx(ctx),
            self.get_operand_e(ctx).with_ctx(ctx)
        )
    }
}

impl Verify for Imm32Op {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

declare_op!(
    /// Represents a Valida program
    ProgramOp,
    "program",
    "valida"
);

impl DisplayWithContext for ProgramOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let region = self.get_region(ctx).with_ctx(ctx).to_string();
        write!(
            f,
            "{} {{\n{}}}",
            self.get_opid().with_ctx(ctx),
            indent::indent_all_by(2, region),
        )
    }
}

impl Verify for ProgramOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        self.verify_interfaces(ctx)?;
        self.get_region(ctx).deref(ctx).verify(ctx)
    }
}

impl ProgramOp {
    /// Attribute key for the main function symbol.
    pub const ATTR_KEY_MAIN_FUNC_SYM: &'static str = "program.main_func_sym";

    /// Create a new [ProgramOP].
    /// The returned programm has a single [crate::region::Region] with a single (BasicBlock)[crate::basic_block::BasicBlock].
    pub fn new(ctx: &mut Context, main_func: FuncOp) -> ProgramOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        let main_func_name = main_func.get_symbol_name(ctx);
        {
            let opref = &mut *op.deref_mut(ctx);
            opref.attributes.insert(
                Self::ATTR_KEY_MAIN_FUNC_SYM,
                StringAttr::create(main_func_name),
            );
        }
        let opop = ProgramOp { op };
        // Create an empty block.
        let region = opop.get_region(ctx);
        let block = BasicBlock::new(ctx, None, vec![]);
        main_func.get_operation().insert_at_back(block, ctx);
        block.insert_at_front(region, ctx);
        opop
    }

    /// Add an [Operation] into this module.
    pub fn add_operation(&self, ctx: &mut Context, op: Ptr<Operation>) {
        self.append_operation(ctx, op, 0)
    }
}

impl OneRegionInterface for ProgramOp {}
impl SingleBlockRegionInterface for ProgramOp {}

declare_op!(
    /// An operation representing a function in Valida
    FuncOp,
    "func",
    "valida"
);

impl FuncOp {
    /// Create a new [FuncOp].
    /// The underlying [Operation] is not linked to a [BasicBlock](crate::basic_block::BasicBlock).
    /// The returned function has a single region with an empty `entry` block.
    pub fn new_unlinked(ctx: &mut Context, name: &str) -> FuncOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        let opop = FuncOp { op };
        // Create an empty entry block.
        #[allow(clippy::expect_used)]
        let region = opop.get_region(ctx);
        let body = BasicBlock::new(ctx, Some("entry".to_string()), vec![]);
        body.insert_at_front(region, ctx);
        opop.set_symbol_name(ctx, name);
        opop
    }

    /// Get the entry block of this function.
    pub fn get_entry_block(&self, ctx: &Context) -> Ptr<BasicBlock> {
        #[allow(clippy::unwrap_used)]
        self.get_region(ctx).deref(ctx).get_head().unwrap()
    }

    /// Get an iterator over all operations.
    pub fn op_iter<'a>(&self, ctx: &'a Context) -> impl Iterator<Item = Ptr<Operation>> + 'a {
        self.get_region(ctx)
            .deref(ctx)
            .iter(ctx)
            .flat_map(|bb| bb.deref(ctx).iter(ctx))
    }
}

impl OneRegionInterface for FuncOp {}
#[cast_to]
impl SymbolOpInterface for FuncOp {}

impl DisplayWithContext for FuncOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let region = self.get_region(ctx).with_ctx(ctx).to_string();
        write!(
            f,
            "{} @{} {{\n{}}}",
            self.get_opid().with_ctx(ctx),
            self.get_symbol_name(ctx),
            indent::indent_all_by(2, region),
        )
    }
}

impl Verify for FuncOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        let op = &*self.get_operation().deref(ctx);
        if op.get_opid() != Self::get_opid_static() {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect OpId".to_string(),
            });
        }
        if op.get_num_results() != 0 || op.get_num_operands() != 0 {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect number of results or operands".to_string(),
            });
        }
        self.verify_interfaces(ctx)?;
        self.get_entry_block(ctx).verify(ctx)?;
        Ok(())
    }
}

pub(crate) fn register(ctx: &mut Context, dialect: &mut Dialect) {
    Imm32Op::register(ctx, dialect);
    ProgramOp::register(ctx, dialect);
    FuncOp::register(ctx, dialect);
}

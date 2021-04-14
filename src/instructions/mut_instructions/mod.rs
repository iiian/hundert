pub mod arith;
pub mod mov;
pub mod neg;
pub mod nop;
pub mod sav;
pub mod swp;

use crate::{
    core::Core,
    resource::{DestType, SrcType},
};
use arith::Arith;
use mov::Mov;
use neg::Neg;
use nop::Nop;
use sav::Sav;
use swp::Swp;

/// An executable instruction associated with a [crate::core::Core]. While it returns no value, it
/// will mutate the underlying [crate::core::Core].
pub trait MutInstr {
    fn execute(&self, core: &mut Core);
}

/// An enum struct of all available instructions.
pub enum Mut {
    /// Arithmetic instructions.
    Arith(Arith),
    /// Data-move instructions.
    Mov(Mov),
    /// Negation instructions.
    Neg(Neg),
    /// No-op instructions.
    Nop(Nop),
    /// Save instructions.
    Sav(Sav),
    /// Swap instructions.
    Swp(Swp),
}

impl Mut {
    /// Creates a new add-to-acc instruction.
    pub fn new_add(src: SrcType) -> Self {
        Mut::Arith(Arith::new_add(src))
    }

    /// Creates a new subtract-from-acc instruction.
    pub fn new_sub(src: SrcType) -> Self {
        Mut::Arith(Arith::new_sub(src))
    }

    /// Creates a new data move instruction.
    pub fn new_mov(src: SrcType, dest: DestType) -> Self {
        Mut::Mov(Mov::new(src, dest))
    }

    /// Creates a new sign-flip-acc instruction.
    pub fn new_neg() -> Self {
        Mut::Neg(Neg::new())
    }

    /// Creates a new overwrite-bak-with-acc instruction.
    pub fn new_sav() -> Self {
        Mut::Sav(Sav::new())
    }

    /// Creates a new swap-acc-with-bak instruction.
    pub fn new_swp() -> Self {
        Mut::Swp(Swp::new())
    }
}

impl MutInstr for Mut {
    /// Executes the given instruction.
    fn execute(&self, core: &mut Core) {
        match self {
            Mut::Arith(instr) => instr.execute(core),
            Mut::Mov(instr) => instr.execute(core),
            Mut::Neg(instr) => instr.execute(core),
            Mut::Nop(instr) => instr.execute(core),
            Mut::Sav(instr) => instr.execute(core),
            Mut::Swp(instr) => instr.execute(core),
        }
    }
}

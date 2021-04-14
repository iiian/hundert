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

pub enum Mut {
    Arith(Arith),
    Mov(Mov),
    Neg(Neg),
    Nop(Nop),
    Sav(Sav),
    Swp(Swp),
}

impl Mut {
    pub fn new_add(src: SrcType) -> Self {
        Mut::Arith(Arith::new_add(src))
    }
    pub fn new_sub(src: SrcType) -> Self {
        Mut::Arith(Arith::new_sub(src))
    }
    pub fn new_mov(src: SrcType, dest: DestType) -> Self {
        Mut::Mov(Mov::new(src, dest))
    }
    pub fn new_neg() -> Self {
        Mut::Neg(Neg::new())
    }
    pub fn new_sav() -> Self {
        Mut::Sav(Sav::new())
    }
    pub fn new_swp() -> Self {
        Mut::Swp(Swp::new())
    }
}

impl MutInstr for Mut {
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

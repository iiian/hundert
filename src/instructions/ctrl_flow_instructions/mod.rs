use crate::{core::Core, resource::SrcType};

mod jcmp;
mod jmp;
mod jro;
mod mut_wrap;

use jcmp::JCmp;
use jmp::Jmp;
use jro::Jro;
use mut_wrap::MutWrap;

use super::mut_instructions::Mut;

pub trait CtrlInstr<'i> {
    fn execute(&self, core: &mut Core) -> &'i Ctrl;
}

pub enum Ctrl<'i> {
    None,
    MutWrap(MutWrap<'i>),
    JCmp(JCmp<'i>),
    Jmp(Jmp<'i>),
    Jro(Jro<'i>),
}

impl<'i> Ctrl<'i> {
    pub fn new_mut_wrap(instr: Mut, next: &'i Ctrl) -> Self {
        Ctrl::MutWrap(MutWrap::new(instr, next))
    }

    pub fn new_jez(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Ctrl::JCmp(JCmp::new_ez(next, branch))
    }

    pub fn new_nz(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Ctrl::JCmp(JCmp::new_nz(next, branch))
    }

    pub fn new_gz(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Ctrl::JCmp(JCmp::new_gz(next, branch))
    }

    pub fn new_lz(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Ctrl::JCmp(JCmp::new_lz(next, branch))
    }

    pub fn new_jmp(next: &'i Ctrl<'i>) -> Self {
        Ctrl::Jmp(Jmp::new(next))
    }

    pub fn new_jro(src: SrcType, index: usize, instrs: &'i [&'i Ctrl<'i>]) -> Self {
        Ctrl::Jro(Jro::new(src, index, instrs))
    }
}

impl<'i> CtrlInstr<'i> for Ctrl<'i> {
    fn execute(&self, core: &mut Core) -> &'i Ctrl {
        match self {
            Ctrl::MutWrap(instr) => instr.execute(core),
            Ctrl::JCmp(instr) => instr.execute(core),
            Ctrl::Jmp(instr) => instr.execute(core),
            Ctrl::Jro(instr) => instr.execute(core),
            Ctrl::None => panic!(),
        }
    }
}

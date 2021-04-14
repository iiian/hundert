use super::{Ctrl, CtrlInstr};
use crate::instructions::mut_instructions::Mut;
use crate::instructions::{ctrl_flow_instructions::Core, mut_instructions::MutInstr};

pub struct MutWrap<'a> {
    mut_instr: Mut,
    next: &'a Ctrl<'a>,
}

impl<'i> MutWrap<'i> {
    pub fn new(instr: Mut, next: &'i Ctrl) -> Self {
        Self {
            mut_instr: instr,
            next,
        }
    }
}

impl<'i> CtrlInstr<'i> for MutWrap<'i> {
    fn execute(&self, core: &mut Core) -> &'i Ctrl {
        self.mut_instr.execute(core);
        self.next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::resource::SrcType;

    #[test]
    #[should_panic]
    fn it_should_not_suck() {
        let i = Mut::new_add(SrcType::Literal(32));
        let sut = MutWrap::new(i, &Ctrl::None);
        let mut core = Core::new();

        let next = sut.execute(&mut core);
        assert_eq!(core.acc, 32);
        next.execute(&mut core);
    }
}

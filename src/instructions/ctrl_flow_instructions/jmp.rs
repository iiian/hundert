use crate::core::Core;

use super::{Ctrl, CtrlInstr};

pub struct Jmp<'i> {
    next: &'i Ctrl<'i>,
}

impl<'i> Jmp<'i> {
    pub fn new(next: &'i Ctrl<'i>) -> Self {
        Self { next }
    }
}

impl<'i> CtrlInstr<'i> for Jmp<'i> {
    fn execute(&self, _: &mut Core) -> &'i Ctrl<'i> {
        self.next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{instructions::mut_instructions::Mut, resource::SrcType};

    #[test]
    fn it_should_provide_the_next_instruction() {
        let mov = Mut::new_mov(SrcType::Literal(16), crate::resource::DestType::Acc);
        let next = Ctrl::new_mut_wrap(mov, &Ctrl::None);
        let jmp = Ctrl::new_jmp(&next);
        let mut core = Core::new();

        jmp.execute(&mut core).execute(&mut core);

        assert_eq!(core.acc, 16);
    }
}

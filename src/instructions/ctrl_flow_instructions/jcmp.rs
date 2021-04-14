use crate::instructions::ctrl_flow_instructions::Core;

use super::{Ctrl, CtrlInstr};

enum CmpStrategy {
    Ez,
    Nz,
    Gz,
    Lz,
}

pub struct JCmp<'i> {
    cmp: CmpStrategy,
    next: &'i Ctrl<'i>,
    branch: &'i Ctrl<'i>,
}

impl<'i> JCmp<'i> {
    pub fn new_ez(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Self {
            cmp: CmpStrategy::Ez,
            next,
            branch,
        }
    }

    pub fn new_nz(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Self {
            cmp: CmpStrategy::Nz,
            next,
            branch,
        }
    }

    pub fn new_gz(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Self {
            cmp: CmpStrategy::Gz,
            next,
            branch,
        }
    }

    pub fn new_lz(next: &'i Ctrl<'i>, branch: &'i Ctrl<'i>) -> Self {
        Self {
            cmp: CmpStrategy::Lz,
            next,
            branch,
        }
    }
}

impl<'i> JCmp<'i> {
    fn will_branch(&self, core: &mut Core) -> bool {
        match self.cmp {
            CmpStrategy::Ez => core.acc == 0,
            CmpStrategy::Nz => core.acc != 0,
            CmpStrategy::Gz => core.acc > 0,
            CmpStrategy::Lz => core.acc < 0,
        }
    }
}

impl<'i> CtrlInstr<'i> for JCmp<'i> {
    fn execute(&self, core: &mut Core) -> &'i Ctrl<'i> {
        if self.will_branch(core) {
            self.branch
        } else {
            self.next
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::mut_instructions::Mut;
    use crate::resource::{DestType, SrcType};

    use super::*;

    #[test]
    fn jez_should_work() {
        let next = Mut::new_mov(SrcType::Literal(64), DestType::Acc);
        let next = Ctrl::new_mut_wrap(next, &Ctrl::None);
        let branch = Mut::new_mov(SrcType::Literal(32), DestType::Acc);
        let branch = Ctrl::new_mut_wrap(branch, &Ctrl::None);
        let jez = JCmp::new_ez(&next, &branch);
        let mut core = Core::new();

        // Because the "branch instruction", ie, the one that is followed when the cond is true is
        // to set the value to 32, we would expect a ground-state core to yield this result.
        jez.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 32);
        // Because the core is no longer ground-state, ie acc != 0, now it should take the "next"
        // instruction, which will set acc to 64.
        jez.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 64);
    }

    #[test]
    fn jnz_should_work() {
        let next = Mut::new_mov(SrcType::Literal(64), DestType::Acc);
        let next = Ctrl::new_mut_wrap(next, &Ctrl::None);
        let branch = Mut::new_mov(SrcType::Literal(32), DestType::Acc);
        let branch = Ctrl::new_mut_wrap(branch, &Ctrl::None);
        let jnz = JCmp::new_nz(&next, &branch);
        let mut core = Core::new();

        jnz.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 64);
        jnz.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 32);
    }

    #[test]
    fn jgz_should_work() {
        let next = Mut::new_mov(SrcType::Literal(64), DestType::Acc);
        let next = Ctrl::new_mut_wrap(next, &Ctrl::None);
        let branch = Mut::new_mov(SrcType::Literal(32), DestType::Acc);
        let branch = Ctrl::new_mut_wrap(branch, &Ctrl::None);
        let jgz = JCmp::new_gz(&next, &branch);
        let mut core = Core::new();

        core.acc = 1;
        jgz.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 32);
        core.acc = -1;
        jgz.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 64);
    }

    #[test]
    fn jlz_should_work() {
        let next = Mut::new_mov(SrcType::Literal(64), DestType::Acc);
        let next = Ctrl::new_mut_wrap(next, &Ctrl::None);
        let branch = Mut::new_mov(SrcType::Literal(32), DestType::Acc);
        let branch = Ctrl::new_mut_wrap(branch, &Ctrl::None);
        let jlz = JCmp::new_lz(&next, &branch);
        let mut core = Core::new();

        core.acc = 1;
        jlz.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 64);
        core.acc = -1;
        jlz.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 32);
    }
}

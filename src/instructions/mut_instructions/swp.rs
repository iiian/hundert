use super::MutInstr;
use crate::core::Core;

pub struct Swp;

impl Swp {
    /// Create a new [Swp] instruction.
    pub fn new() -> Self {
        Self {}
    }
}

impl MutInstr for Swp {
    /// acc <-> bak in [Core](crate::core::Core) memory.
    fn execute(&self, core: &mut Core) {
        std::mem::swap(&mut core.acc, &mut core.bak);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_swap() {
        let mut core = Core::new();
        core.acc = 16;
        core.bak = 32;
        let i = Swp::new();

        i.execute(&mut core);

        assert_eq!(core.acc, 32);
        assert_eq!(core.bak, 16);
    }
}

use super::MutInstr;
use crate::core::Core;

pub struct Sav;

impl Sav {
    /// Create a new [Sav] instruction.
    pub fn new() -> Self {
        Self
    }
}

impl MutInstr for Sav {
    /// Write [Core.acc](crate::core::Core) -> [Core.bak](crate::core::Core).
    fn execute(&self, core: &mut Core) {
        core.bak = core.acc;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_overwrite() {
        let mut core = Core::new();
        core.acc = 16;
        core.bak = 32;
        let i = Sav::new();

        i.execute(&mut core);

        assert_eq!(core.acc, 16);
        assert_eq!(core.bak, 16);
    }
}

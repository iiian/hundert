use super::MutInstr;

pub struct Neg {}

impl Neg {
    pub fn new() -> Self {
        Self {}
    }
}

impl MutInstr for Neg {
    fn execute(&self, core: &mut crate::core::Core) {
        core.acc = -1 * core.acc;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::Core;

    #[test]
    fn it_should_negate() {
        let mut core = Core::new();
        core.acc = 32;
        let i = Neg::new();

        i.execute(&mut core);

        assert_eq!(core.acc, -32);
    }
}

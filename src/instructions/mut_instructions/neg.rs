use super::MutInstr;

/// A negation instruction. Flips the sign on the acc register of a [Core](crate::core::Core).
pub struct Neg {}

impl Neg {
    /// Create a new [Neg].
    pub fn new() -> Self {
        Self {}
    }
}

impl MutInstr for Neg {
    /// Flip the sign on [Core.acc](crate::core::Core).
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

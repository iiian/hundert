use super::MutInstr;
use crate::resource::{DestType, SrcType};

pub struct Arith {
    src: SrcType,
    is_addition: bool,
}

impl Arith {
    /// Creates an add instruction.
    pub fn new_add(src: SrcType) -> Self {
        Self {
            src,
            is_addition: true,
        }
    }

    /// Creates a subtraction instruction.
    pub fn new_sub(src: SrcType) -> Self {
        Self {
            src,
            is_addition: false,
        }
    }
}

impl MutInstr for Arith {
    fn execute(&self, core: &mut crate::core::Core) {
        let val = self.src.read(core);

        core.acc = if self.is_addition {
            core.acc + val
        } else {
            core.acc - val
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::Core;

    #[test]
    fn it_should_add_literal() {
        let mut core = Core::new();
        core.acc = 16;
        let i = Arith::new_add(SrcType::Literal(16));

        i.execute(&mut core);

        assert_eq!(core.acc, 32);
    }

    #[test]
    fn it_should_del_literal() {
        let mut core = Core::new();
        core.acc = 16;
        let i = Arith::new_sub(SrcType::Literal(10));

        i.execute(&mut core);

        assert_eq!(core.acc, 6);
    }

    #[test]
    fn add_should_double_given_acc() {
        let mut core = Core::new();
        core.acc = 8;
        let i = Arith::new_add(SrcType::Resource(DestType::Acc));

        i.execute(&mut core);

        assert_eq!(core.acc, 16);
    }

    #[test]
    fn sub_should_zero_out_given_acc() {
        let mut core = Core::new();
        core.acc = 8;
        let i = Arith::new_sub(SrcType::Resource(DestType::Acc));

        i.execute(&mut core);

        assert_eq!(core.acc, 0);
    }

    #[test]
    fn add_should_nop_on_nil() {
        let mut core = Core::new();
        core.acc = 16;
        let i = Arith::new_add(SrcType::Resource(DestType::Nil));

        i.execute(&mut core);

        assert_eq!(core.acc, 16);
    }
}

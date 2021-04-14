use crate::core::Core;
use crate::instructions::mut_instructions::MutInstr;
use crate::resource::{DestType, SrcType};

/// An instruction which moves a given source type value to a given dest type value.
pub struct Mov {
    src: SrcType,
    dest: DestType,
}

impl Mov {
    /// Create a new [Mov] [Instruction] instance.
    pub fn new(src: SrcType, dest: DestType) -> Self {
        Self { src, dest }
    }
}

impl MutInstr for Mov {
    /// Attempts to copy information available at the source specified by `src`, to the storage location at `dest`.
    fn execute(&self, core: &mut Core) {
        let src_val = match self.src {
            SrcType::Resource(DestType::Acc) => core.acc,
            SrcType::Resource(DestType::Nil) => return,
            SrcType::Literal(n) => n,
        };

        match self.dest {
            DestType::Acc => core.acc = src_val,
            DestType::Nil => return,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_move_btw_acc_bak() {
        let mut core = Core::new();
        core.acc = 10;

        let i = Mov::new(SrcType::Literal(60), DestType::Acc);
        i.execute(&mut core);

        assert_eq!(core.acc, 60);
    }
}

use crate::core::Core;
use crate::instructions::mut_instructions::MutInstr;
use crate::resource::{DestType, SrcType};

/// An instruction which moves a given source type value to a given dest type value.
pub struct Mov {
    src: SrcType,
    dest: DestType,
}

impl Mov {
    /// Create a new [Mov] instance.
    pub fn new(src: SrcType, dest: DestType) -> Self {
        Self { src, dest }
    }
}

impl MutInstr for Mov {
    /// Attempts to copy information available at the source specified by `src`, to the storage location at `dest`.
    /// A `src` may be either a literal number, nil (which will evaluate to zero), acc, or any of the
    /// four companion ports of a [Core](crate::core::Core). A dest may be `acc`, or any of the
    /// companion ports.
    fn execute(&self, core: &mut Core) {
        let value = self.src.read(core);
        self.dest.write(value, core);
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

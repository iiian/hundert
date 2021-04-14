use super::MutInstr;
use crate::core::Core;

pub struct Nop;

impl MutInstr for Nop {
    fn execute(&self, _: &mut Core) {}
}

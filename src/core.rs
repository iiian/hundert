/// A collection of registers and networking ports. [MutInstructions](crate::instructions::instruction::MutInstruction) execute to change the stored
/// values in registers, and pass messages between ports.
pub struct Core {
    pub acc: i16,
    pub bak: i16,
}

impl Core {
    pub fn new() -> Self {
        Core { acc: 0, bak: 0 }
    }
}

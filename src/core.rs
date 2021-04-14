use crate::port::Port;

/// A collection of registers and networking ports. [MutInstrs](crate::instructions::mut_instructions::MutInstr) execute to change the stored
/// values in registers, and pass messages between ports.
/// [CtrlInstrs](crate::instructions::ctrl_flow_instructions::CtrlInstr) execute by looking at
/// [Core] registers and making branching decisions according to state.
pub struct Core {
    pub acc: i16,
    pub bak: i16,
    pub left: Port,
    pub right: Port,
    pub up: Port,
    pub down: Port,
}

impl Core {
    pub fn new() -> Self {
        Self {
            acc: 0,
            bak: 0,
            left: Port::default(),
            right: Port::default(),
            up: Port::default(),
            down: Port::default(),
        }
    }
}

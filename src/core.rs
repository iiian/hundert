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
    /// Create a new [Core] with no peer bindings. Use [bind_up_down](crate::port::bind_up_down)
    /// and/or [bind_left_right](crate::port::bind_left_right) to specify peer relationships
    /// between cores.
    ///
    /// ```
    /// use crate::core::Core;
    /// use crate::port::bind_up_down;
    ///
    /// let mut top = Core::new();
    /// let mut bottom = Core::new();
    /// bind_up_down(top, bottom);
    ///
    /// // now these two cores may speak with one other.
    /// let move_down = Mut::new_mov(SrcType::Literal(16), DestType::Down);
    /// let add_up = Mut::new_add(SrcType::Resource(DestType::Up));
    ///
    /// // note that these two actions may occur in separate threads.
    /// move_down.execute(&mut top);
    /// add_up.execute(&mut bottom);
    ///
    /// assert_eq!(bottom.acc, 16);
    /// ```
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

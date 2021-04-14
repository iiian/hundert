use crate::core::Core;

/// The destination types of a move command.
pub enum DestType {
    /// Reference the `acc` property of a [crate::core::Core].
    Acc,
    /// Throw it away.
    Nil,
    /// Write to peer above.
    Up,
    /// Write to peer below.
    Down,
    /// Write to peer on left.
    Left,
    /// Write to peer on right.
    Right,
    // Any -- eeesh... this one is gonna SUCK
    // Last -- core will need to expose something about which was the last port to be hit?
}

impl DestType {
    /// Send a value to a core memory destination.
    pub fn write(&self, value: i16, core: &mut Core) {
        match self {
            DestType::Acc => core.acc = value,
            DestType::Nil => {}
            DestType::Up => core.up.send(value),
            DestType::Down => core.down.send(value),
            DestType::Left => core.left.send(value),
            DestType::Right => core.right.send(value),
        }
    }
}

/// The source types of a move command.
pub enum SrcType {
    /// Move an actual number.
    Literal(i16),
    /// Move from a storage location.
    Resource(DestType),
}

impl SrcType {
    /// Read a value from a core memory source.
    pub fn read(&self, core: &Core) -> i16 {
        match self {
            &SrcType::Literal(val) => val,
            SrcType::Resource(DestType::Acc) => core.acc,
            SrcType::Resource(DestType::Up) => core.up.get(),
            SrcType::Resource(DestType::Down) => core.down.get(),
            SrcType::Resource(DestType::Left) => core.left.get(),
            SrcType::Resource(DestType::Right) => core.right.get(),
            SrcType::Resource(DestType::Nil) => 0,
        }
    }
}

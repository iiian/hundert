/// The destination types of a move command.
pub enum DestType {
    /// Reference the `acc` property of a [Core].
    Acc,
    /// Throw it away.
    Nil,
    // Left, Right, Up, Down
    // Any -- eeesh... this one is gonna SUCK
    // Last -- core will need to expose something about which was the last port to be hit?
}

/// The source types of a move command.
pub enum SrcType {
    /// Move an actual number.
    Literal(i16),
    /// Move from a storage location.
    Resource(DestType),
}

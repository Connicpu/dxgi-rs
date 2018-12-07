#[enum_flags(u32)]
/// Flags for customizing behavior of a window association in DXGI
pub enum WindowAssociationFlags {
    /// No flags specified (default behavior).
    NONE = 0,

    /// Prevent DXGI from monitoring an applications message queue; this makes
    /// DXGI unable to respond to mode changes.
    NO_WINDOW_CHANGES = 1 << 0,
    
    /// Prevent DXGI from responding to an alt-enter sequence.
    NO_ALT_ENTER = 1 << 1,

    /// Prevent DXGI from responding to a print-screen key.
    NO_PRINT_SCREEN = 1 << 2,
}

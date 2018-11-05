#[enum_flags(u32)]
/// Determines CPU memory access to mapped surface memory.
pub enum MapFlags {
    /// Allow CPU read access.
    READ = 1,
    /// Allow CPU write access.
    WRITE = 2,
    /// Discard the previous contents of a resource when it is mapped.
    DISCARD = 4,
}

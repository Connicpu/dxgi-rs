#[auto_enum::auto_enum(u32, checked)]
/// Indicates how the back buffers should be rotated to fit the physical
/// rotation of a monitor.
pub enum ModeRotation {
    /// Unspecified, the meaning of this is contextual to the API
    /// currently in use.
    Unspecified = 0,

    /// No rotation is performed
    Identity = 1,

    /// Rotate 90 degrees
    Rotate90 = 2,

    /// Rotate 180 degrees
    Rotate180 = 3,

    /// Rotate 270 degrees
    Rotate270 = 4,
}

#[auto_enum(u32, checked)]
/// Indicates how an image is stretched to fit a given monitor's resolution.
pub enum ModeScaling {
    /// Unspecified, the meaning of this is contextual to the API
    /// currently in use.
    Unspecified = 0,

    /// The image is not scaled and is instead centered in the middle of the
    /// screen when the image is smaller than the resolution.
    Centered = 1,

    /// Image will be stretched to fill entire monitor.
    Stretched = 2,
}

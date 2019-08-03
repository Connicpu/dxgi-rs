#[auto_enum::auto_enum(u32, checked)]
/// Indicates the method the raster uses to create an image on a surface.
pub enum ModeScanlineOrder {
    /// Unspecified, the meaning of this is contextual to the API
    /// currently in use.
    Unspecified = 0,

    /// The image is created from the first scanline to the last without
    /// skipping any.
    Progressive = 1,

    /// The image is created beginning with the upper field.
    UpperFieldFirst = 2,

    /// The image is created beginning with the lower field.
    LowerFieldFirst = 3,
}

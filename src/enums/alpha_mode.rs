#[auto_enum(u32, checked)]
/// Identifies the alpha transparency behavior of a surface
pub enum AlphaMode {
    /// Unspecified, the meaning of this is contextual to the API
    /// currently in use.
    Unspecified = 0,
    
    /// Indicates that the transparency behavior is premultiplied. Each color
    /// is first scaled by the alpha value. The alpha value itself is the same
    /// in both straight and premultiplied alpha. Typically, no color channel
    /// value is greater than the alpha channel value. If a color channel value
    /// in a premultiplied format is greater than the alpha channel, the
    /// standard source-over blending math results in an additive blend.
    Premultiplied = 1,

    /// Indicates that the transparency behavior is not premultiplied. The
    /// alpha channel indicates the transparency of the color.
    Straight = 2,

    /// Indicates to ignore the transparency behavior.
    Ignore = 3,
}

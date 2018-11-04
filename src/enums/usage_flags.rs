#[enum_flags(u32)]
/// Flags for surface and resource creation options.
/// 
/// These flag options are used when you create a swap chain to describe the
/// surface usage and CPU access options for the back buffer of a swap chain.
/// You can't use the `SHARED`, `DISCARD_ON_PRESENT`, and `READ_ONLY` values
/// as input to create a swap chain. However, DXGI can set `DISCARD_ON_PRESENT`
/// and `READ_ONLY` for some of the swap chain's back buffers on the
/// application's behalf.
/// 
/// These flags are also used when you create a [`Surface`][1].
/// 
/// [1]: ../struct.Surface.html
pub enum UsageFlags {
    /// The surface or resource is used as a back buffer. You don’t need to
    /// pass `BACK_BUFFER` when you create a swap chain. But you can determine
    /// whether a resource belongs to a swap chain when you call `get_usage`
    /// and get `BACK_BUFFER`.
    BACK_BUFFER = 1 << (2 + 4),

    /// This flag is for internal use only.
    DISCARD_ON_PRESENT = 1 << (5 + 4),

    /// Use the surface or resource for reading only.
    READ_ONLY = 1 << (4 + 4),

    /// Use the surface or resource as an output render target.
    RENDER_TARGET_OUTPUT = 1 << (1 + 4),

    /// Use the surface or resource as an input to a shader.
    SHADER_INPUT = 1 << (0 + 4),

    /// Share the surface or resource.
    SHARED = 1 << (3 + 4),
    
    /// Use the surface or resource for unordered access.
    UNORDERED_ACCESS = 1 << (6 + 4),
}

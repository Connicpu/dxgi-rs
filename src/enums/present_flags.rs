#[enum_flags(u32)]
/// Options for presenting frames to the output.
/// 
/// The default (`NONE`) will present a frame from each buffer (starting with
/// the current buffer) to the output.
pub enum PresentFlags {
    /// No flags specified (default behavior).
    NONE = 0,

    /// Do not present the frame to the output. The status of the swap chain
    /// will be tested and appropriate errors returned. `TEST` is intended for
    /// use only when switching from the idle state; do not use it to determine
    /// when to switch to the idle state because doing so can leave the swap
    /// chain unable to exit full-screen mode.
    TEST = 0x001,

    /// Present a frame from the current buffer to the output. Use this flag so
    /// that the presentation can use vertical-blank synchronization instead of
    /// sequencing buffers in the chain in the usual manner.
    /// 
    /// ***Note:***
    /// If the calling application sets the `DO_NOT_SEQUENCE` flag on the first
    /// present operation (that is, when there is no current buffer), the
    /// runtime ignores that present operation and does not call the driver.
    DO_NOT_SEQUENCE = 0x002,

    /// Specifies that the runtime will discard outstanding queued presents.
    RESTART = 0x004,

    /// Specifies that the runtime will fail the presentation with the
    /// [`DXGI_ERROR_WAS_STILL_DRAWING`][1] error code if the calling thread is
    /// blocked; the runtime returns [`DXGI_ERROR_WAS_STILL_DRAWING`][1] instead of
    /// sleeping until the dependency is resolved.
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    /// 
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/direct3ddxgi/dxgi-error#DXGI_ERROR_WAS_STILL_DRAWING
    DO_NOT_WAIT = 0x008,

    /// Indicates that presentation content will be shown only on the
    /// particular output. The content will not be visible on other outputs.
    /// For example, if the user tries to relocate video content on another
    /// output, the video content will not be visible.
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    /// 
    /// ***Note:***
    /// This flag should only be used with swap effect [`FlipSequential`][1]
    /// or [`FlipDiscard`][2]. The use of this flag with other swap effects is
    /// being deprecated, and may not work in future versions of Windows. 
    /// 
    /// [1]: enum.SwapEffect.html#variant.FlipSequential
    /// [2]: enum.SwapEffect.html#variant.FlipDiscard
    RESTRICT_TO_OUTPUT = 0x010,

    /// Indicates that if the stereo present must be reduced to mono, right-eye
    /// viewing is used rather than left-eye viewing.
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    STEREO_PREFER_RIGHT = 0x020,

    /// Indicates that the presentation should use the left buffer as a mono
    /// buffer. An application calls the
    /// [`SwapChain::is_temporary_mono_supported`][1] method to determine
    /// whether a swap chain supports "temporary mono".
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    /// 
    /// [1]: ../struct.SwapChain.html#method.is_temporary_mono_supported
    STEREO_TEMPORARY_MONO = 0x040,

    /// This flag must be set by media apps that are currently using a custom
    /// present duration (custom refresh rate).
    /// 
    /// ***Note:***
    /// This value is supported starting in Windows 8.1.
    USE_DURATION = 0x100,

    /// Allowing tearing is a requirement of variable refresh rate displays.
    /// 
    /// The conditions for using `ALLOW_TEARING` during Present are as follows:
    /// 
    ///  * The swap chain must be created with the `ALLOW_TEARING` flag.
    ///  * The sync interval passed in to [`present`][1] must be 0.
    ///  * The `ALLOW_TEARING` flag cannot be used in an application that is
    ///    currently in full screen exclusive mode. It can only be used in
    ///    windowed mode. To use this flag in full screen Win32 apps, the
    ///    application should present to a fullscreen borderless window and
    ///    disable automatic ALT+ENTER fullscreen switching.
    /// 
    /// Calling [`present`][1] with this flag and not meeting the conditions
    /// above will result in a [`DXGI_ERROR_INVALID_CALL`][2] error being returned to
    /// the calling application.
    /// 
    /// [1]: ../struct.SwapChain.html#method.present
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/direct3ddxgi/dxgi-error#DXGI_ERROR_INVALID_CALL
    ALLOW_TEARING = 0x200,
}

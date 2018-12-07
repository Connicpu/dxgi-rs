#[enum_flags(u32)]
/// Options for swap-chain behavior.
pub enum SwapChainFlags {
    /// No flags specified.
    NONE = 0,

    /// Set this flag to turn off automatic image rotation; that is, do not
    /// perform a rotation when transferring the contents of the front buffer
    /// to the monitor. Use this flag to avoid a bandwidth penalty when an
    /// application expects to handle rotation. This option is valid only
    /// during full-screen mode.
    /// 
    /// [More Information][1]
    /// 
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/api/dxgi/ne-dxgi-dxgi_swap_chain_flag
    NONPREROTATED = 1,

    /// Set this flag to enable an application to switch modes by calling
    /// [`SwapChain::resize_target`][1]. When switching from windowed to
    /// full-screen mode, the display mode (or monitor resolution) will be
    /// changed to match the dimensions of the application window.
    /// 
    /// [1]: ../struct.SwapChain.html#method.resize_target
    ALLOW_MODE_SWITCH = 2,

    /// Set this flag to enable an application to render using GDI on a swap
    /// chain or a surface. This will allow the application to call
    /// [`IDXGISurface1::GetDC`][1] on the 0th back buffer or a surface.
    /// 
    /// [1]: https://msdn.microsoft.com/b148d2b4-36a2-46b9-8a98-9f3c478549a4
    GDI_COMPATIBLE = 4,

    /// Set this flag to indicate that the swap chain might contain protected
    /// content; therefore, the operating system supports the creation of the
    /// swap chain only when driver and hardware protection is used. If the
    /// driver and hardware do not support content protection, the call to
    /// create a resource for the swap chain fails.
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    RESTRICTED_CONTENT = 8,

    /// Set this flag to indicate that shared resources that are created within
    /// the swap chain must be protected by using the driver’s mechanism for
    /// restricting access to shared surfaces.
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    RESTRICT_SHARED_RESOURCE_DRIVER = 16,

    /// Set this flag to restrict presented content to the local displays.
    /// Therefore, the presented content is not accessible via remote
    /// accessing or through the [desktop duplication APIs][1].
    /// 
    /// This flag supports the window content protection features of Windows.
    /// Applications can use this flag to protect their own onscreen window
    /// content from being captured or copied through a specific set of public
    /// operating system features and APIs.
    /// 
    /// If you use this flag with windowed (HWND or IWindow) swap chains where
    /// another process created the HWND, the owner of the HWND must use the
    /// [`SetWindowDisplayAffinity`][2] function appropriately in order to
    /// allow calls to [`present`][3] to succeed.
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    /// 
    /// [1]: https://msdn.microsoft.com/02C4EC3D-D97F-4CFC-ABF5-03B44CE6A658
    /// [2]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-setwindowdisplayaffinity
    /// [3]: ../struct.SwapChain.html#method.present
    DISPLAY_ONLY = 32,

    /// Set this flag to create a waitable object you can use to ensure
    /// rendering does not begin while a frame is still being presented. When
    /// this flag is used, the swapchain's latency must be set with the
    /// [`IDXGISwapChain2::SetMaximumFrameLatency`][1] API instead of
    /// [`IDXGIDevice1::SetMaximumFrameLatency`][2] (currently not available
    /// directly in this crate, but may be achieved through direct use of
    /// DXGI pointers).
    /// 
    /// **Note**
    /// This enumeration value is supported starting with Windows 8.1.
    /// 
    /// [1]: https://msdn.microsoft.com/AF3F03F2-38B4-474A-8A66-86A93D776EA0
    /// [2]: https://msdn.microsoft.com/ea477f33-2dba-44ac-9b47-8fd2ce6cec30
    FRAME_LATENCY_WAITABLE_OBJECT = 64,

    /// Set this flag to create a swap chain in the foreground layer for
    /// multi-plane rendering. This flag can only be used with CoreWindow swap
    /// chains. Apps should not create foreground swap chains if
    /// [`IDXGIOutput2::SupportsOverlays`][1] indicates that hardware support
    /// for overlays is not available.
    /// 
    /// Note that [`resize_buffers`][2] cannot be used to add or remove this flag.
    /// 
    /// **Note**
    /// This enumeration value is supported starting with Windows 8.1.
    /// 
    /// [1]: https://msdn.microsoft.com/BC9CD287-CD89-4D0C-ADE3-EAA60D5FEAAD
    /// [2]: ../struct.SwapChain.html#method.resize_buffers
    FOREGROUND_LAYER = 128,

    /// Set this flag to create a swap chain for full-screen video.
    /// 
    /// **Note**
    /// This enumeration value is supported starting with Windows 8.1.
    FULLSCREEN_VIDEO = 256,

    /// Set this flag to create a swap chain for YUV video.
    /// 
    /// **Note**
    /// This enumeration value is supported starting with Windows 8.1.
    YUV_VIDEO = 512,

    /// Indicates that the swap chain should be created such that all
    /// underlying resources can be protected by the hardware. Resource
    /// creation will fail if hardware content protection is not supported.
    /// 
    /// This flag has the following restrictions:
    ///  * This flag can only be used with swap effect [`FlipSequential`][1].
    /// 
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    /// 
    /// **Note**
    /// Creating a swap chain using this flag does not automatically guarantee
    /// that hardware protection will be enabled for the underlying allocation.
    /// Some implementations require that the DRM components are first
    /// initialized prior to any guarantees of protection.
    /// 
    /// </div>
    /// 
    /// **Note**
    /// This enumeration value is supported starting with Windows 8.1.
    /// 
    /// [1]: enum.SwapEffect.html#variant.FlipSequential
    HW_PROTECTED = 1024,

    /// Tearing support is a requirement to enable displays that support
    /// variable refresh rates to function properly when the application
    /// presents a swap chain tied to a full screen borderless window. Win32
    /// apps can already achieve tearing in fullscreen exclusive mode by
    /// calling [`SwapChain::set_fullscreen_state`][1]`(true)`, but the recommended
    /// approach for Win32 developers is to use this tearing flag instead.
    /// This flag requires the use of a `Flip*` swap effect.
    /// 
    /// To check for hardware support of this feature, refer to
    /// [`IDXGIFactory5::CheckFeatureSupport`][2]. For usage information
    /// refer to [`SwapChain::present`][3] and [`PresentFlags`][4].
    /// 
    /// [1]: ../struct.SwapChain.html#method.set_fullscreen_state
    /// [2]: https://msdn.microsoft.com/959F83F8-ADC6-4609-8F63-BEDDFC2EF088
    /// [3]: ../struct.SwapChain.html#method.present
    /// [4]: struct.PresentFlags.html
    ALLOW_TEARING = 2048,

    /// This flag does not have any official documentation on MSDN. If someone
    /// knows what information should go here, please open a PR on the [repo][1]!
    /// 
    /// [1]: https://github.com/Connicpu/dxgi-rs
    RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS = 4096,
}

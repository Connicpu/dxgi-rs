#[auto_enum(u32, checked)]
/// Options for handling pixels in a display surface after calling
/// [`SwapChain::present`][1].
/// 
/// [1]: ../struct.SwapChain.html#method.present
pub enum SwapEffect {
    /// Use this flag to specify the bit-block transfer (bitblt) model and to
    /// specify that DXGI discard the contents of the back buffer after you
    /// call [`SwapChain::present`][1]. This flag is valid for a swap chain
    /// with more than one back buffer, although, applications only have read
    /// and write access to buffer 0. Use this flag to enable the display
    /// driver to select the most efficient presentation technique for the
    /// swap chain.
    /// 
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    /// 
    /// **Note**
    /// There are differences between full screen exclusive and full screen
    /// UWP. If you are porting a Direct3D 11 application to UWP on a Windows
    /// PC, be aware that the use of `Discard` when creating swap chains does
    /// not behave the same way in UWP as it does in Win32, and its use may be
    /// detrimental to GPU performance.
    /// 
    /// This is because UWP applications are forced into `Flip*` swap modes
    /// (even if other swap modes are set), because this reduces the
    /// computation time used by the memory copies originally done by the older
    /// bitblt model.
    /// 
    /// The recommended approach is to manually convert DX11 Discard swap
    /// chains to use flip models within UWP, using [`FlipDiscard`][2] instead
    /// of `Discard` where possible. See [this article][3] for more information.
    /// 
    /// </div>
    /// 
    /// [1]: ../struct.SwapChain.html#method.present
    /// [2]: #variant.FlipDiscard
    /// [3]: https://msdn.microsoft.com/B6B92F4F-B1D0-40B9-987D-F0C0F2CC7AD1
    Discard = 0,

    /// Use this flag to specify the bitblt model and to specify that DXGI
    /// persist the contents of the back buffer after you call
    /// [`SwapChain::present`][1]. Use this option to present the contents of
    /// the swap chain in order, from the first buffer (buffer 0) to the last
    /// buffer. This flag cannot be used with multisampling.
    /// 
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    /// 
    /// **Note**
    /// For best performance, use [`FlipSequential`][2] instead of
    /// `Sequential`. See [this article][3] for more information.
    /// 
    /// </div>
    /// 
    /// [1]: ../struct.SwapChain.html#method.present
    /// [2]: #variant.FlipSequential
    /// [3]: https://msdn.microsoft.com/B6B92F4F-B1D0-40B9-987D-F0C0F2CC7AD1
    Sequential = 1,

    /// Use this flag to specify the flip presentation model and to specify
    /// that DXGI persist the contents of the back buffer after you call
    /// [`SwapChain::present`][1]. This flag cannot be used with multisampling.
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    /// 
    /// [1]: ../struct.SwapChain.html#method.present
    FlipSequential = 3,

    /// Use this flag to specify the flip presentation model and to specify
    /// that DXGI discard the contents of the back buffer after you call
    /// [`SwapChain::present`][1]. This flag cannot be used with multisampling
    /// and partial presentation. See [DXGI 1.4 Improvements][2].
    /// 
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    /// 
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    /// 
    /// **Note**
    /// Windows Store apps must use `FlipSequential` or `FlipDiscard`.
    /// 
    /// </div>
    /// 
    /// [1]: ../struct.SwapChain.html#method.present
    /// [2]: https://msdn.microsoft.com/DEA901EA-B0F9-41D9-802C-ED1D6A7888E0
    FlipDiscard = 4,
}

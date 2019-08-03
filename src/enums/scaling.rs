#[auto_enum::auto_enum(u32, checked)]
/// Identifies resize behavior when the back-buffer size does not match the
/// size of the target output.
pub enum Scaling {
    /// Directs DXGI to make the back-buffer contents scale to fit the
    /// presentation target size.
    Stretch = 0,

    /// Directs DXGI to make the back-buffer contents appear without any
    /// scaling when the presentation target size is not equal to the
    /// back-buffer size. The top edges of the back buffer and presentation
    /// target are aligned together. If the WS_EX_LAYOUTRTL style is associated
    /// with the HWND handle to the target output window, the right edges of
    /// the back buffer and presentation target are aligned together;
    /// otherwise, the left edges are aligned together. All target area outside
    /// the back buffer is filled with window background color.
    ///
    /// This value specifies that all target areas outside the back buffer of a
    /// swap chain are filled with the background color that you specify in a
    /// call to [`SwapChain::set_background_color`][1].
    ///
    /// ### `None` Remarks
    ///
    /// This value is supported only for flip presentation model swap chains
    /// that you create with the [`FlipSequential`][2] value.
    ///
    /// [1]: ../struct.SwapChain.html#method.set_background_color
    /// [2]: enum.SwapEffect.html#variant.FlipSequential
    None = 1,

    /// Directs DXGI to make the back-buffer contents scale to fit the
    /// presentation target size, while preserving the aspect ratio of the
    /// back-buffer. If the scaled back-buffer does not fill the presentation
    /// area, it will be centered with black borders.
    ///
    /// This constant is supported on Windows Phone 8 and Windows 10.
    ///
    /// Note that with legacy Win32 window swapchains, this works the same as
    /// [`Stretch`][1].
    ///
    /// ### `AspectRatioStretch` Remarks
    ///
    /// This value will prefer to use a horizontal fill, otherwise it
    /// will use a vertical fill, using the following logic:
    ///
    /// ```
    /// # use std::cmp::{max, min};
    /// # let back_buffer_width = 800u32;
    /// # let back_buffer_height = 480u32;
    /// # let output_width = 800u32;
    /// # let output_height = 480u32;
    /// # struct Rect { left: u32, top: u32, right: u32, bottom: u32 }
    /// # let mut rect = Rect { left: 0, top: 0, right: 0, bottom: 0 };
    /// let aspect_ratio = back_buffer_width as f32 / back_buffer_height as f32;
    ///
    /// // Horizontal fill
    /// let mut scaled_width = output_width as f32;
    /// let mut scaled_height = output_width as f32 / aspect_ratio;
    /// if scaled_height >= output_height as f32 {
    ///     // Do vertical fill
    ///     scaled_width = output_height as f32 * aspect_ratio;
    ///     scaled_height = output_height as f32;
    /// }
    ///
    /// let offset_x = (output_width as f32 - scaled_width) * 0.5;
    /// let offset_y = (output_height as f32 - scaled_height) * 0.5;
    ///
    /// rect.left = max(offset_x as u32, 0);
    /// rect.top = max(offset_y as u32, 0);
    /// rect.right = min((offset_x + scaled_width) as u32, output_width);
    /// rect.bottom = min((offset_y + scaled_height) as u32, output_height);
    /// ```
    ///
    /// Note that output_width and output_height are the pixel sizes of the
    /// presentation target size. In the case of CoreWindow, this requires
    /// converting the logical_width and logical_height values from DIPS to
    /// pixels using the window's DPI property.
    ///
    /// [1]: #variant.Stretch
    AspectRatioStretch = 2,
}

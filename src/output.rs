use device::Device;
use enums::{Format, ModeRotation, ModeScaling, ModeScanlineOrder};
use error::Error;
use ratio::Ratio;
use surface::Surface;

use std::ffi::OsString;
use std::fmt;
use std::mem;
use std::ptr;

use checked_enum::UncheckedEnum;
use math2d::Recti;
use winapi::shared::dxgi::{IDXGIOutput, DXGI_FRAME_STATISTICS, DXGI_OUTPUT_DESC};
use winapi::shared::dxgitype::{
    DXGI_GAMMA_CONTROL, DXGI_GAMMA_CONTROL_CAPABILITIES, DXGI_MODE_DESC, DXGI_RATIONAL, DXGI_RGB,
    DXGI_RGBA,
};
use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::HMONITOR;
use winapi::shared::winerror::{DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, S_OK};
use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;
use wio::wide::FromWide;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// Represents an adapter output (such as a monitor).
pub struct Output {
    ptr: ComPtr<IDXGIOutput>,
}

impl Output {
    #[inline]
    /// Get a description of the output.
    pub fn get_desc(&self) -> OutputDesc {
        unsafe {
            let mut desc = mem::uninitialized();

            let result = self.ptr.GetDesc(&mut desc);
            assert!(result >= 0);

            OutputDesc { desc }
        }
    }

    #[inline]
    /// Gets the display modes that match the requested format and other input
    /// options.
    pub fn get_modes(&self, format: Format) -> Result<Vec<Mode>, Error> {
        unsafe {
            let mut buf: Vec<Mode> = Vec::new();
            loop {
                let mut len = 0;
                let ptr = ptr::null_mut();
                let hr = self.ptr.GetDisplayModeList(format as u32, 2, &mut len, ptr);
                Error::map(hr, ())?;

                buf.reserve_exact(len as usize);

                let ptr = buf.as_mut_ptr() as *mut DXGI_MODE_DESC;
                let hr = self.ptr.GetDisplayModeList(format as u32, 2, &mut len, ptr);
                match hr {
                    S_OK => {
                        buf.set_len(len as usize);
                        return Ok(buf);
                    }
                    DXGI_ERROR_MORE_DATA => continue,
                    DXGI_ERROR_NOT_CURRENTLY_AVAILABLE => return Err(Error(hr)),
                    _ => unreachable!(),
                }
            }
        }
    }

    #[inline]
    /// Finds the display mode that most closely matches the requested display
    /// mode.
    pub fn find_closest_matching_mode(
        &self,
        mode: &Mode,
        device: Option<&Device>,
    ) -> Result<Mode, Error> {
        unsafe {
            let dev = device
                .map(|d| d.get_raw() as *mut IUnknown)
                .unwrap_or(ptr::null_mut());

            let mut matching: Mode = mem::uninitialized();
            let hr = self
                .ptr
                .FindClosestMatchingMode(&mode.desc, &mut matching.desc, dev);

            Error::map(hr, matching)
        }
    }

    #[inline]
    /// Halt a thread until the next vertical blank occurs.
    pub fn wait_for_vblank(&self) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.WaitForVBlank();
            Error::map(hr, ())
        }
    }

    #[inline]
    /// Takes ownership of an output. When you are finished with the output,
    /// call `release_ownership`.
    ///
    /// `take_ownership` should not be called directly by applications, since
    /// results will be unpredictable. It is called implicitly by the DXGI swap
    /// chain object during full-screen transitions, and should not be used as
    /// a substitute for swap-chain methods.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// This method is marked as unsafe because it is not clear what the
    /// implications of calling this method are, and therefore an application
    /// should be certain this is the action they would like to take.
    ///
    /// </div>
    ///
    /// If a Windows Store app uses `take_ownership`, it fails with
    /// [`DXGI_ERROR_NOT_CURRENTLY_AVAILABLE`][1].
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/direct3ddxgi/dxgi-error#DXGI_ERROR_NOT_CURRENTLY_AVAILABLE
    pub unsafe fn take_ownership(&self, device: &Device, exclusive: bool) -> Result<(), Error> {
        let dev = device.get_raw();
        let hr = self.ptr.TakeOwnership(dev as *mut _, exclusive as BOOL);
        Error::map(hr, ())
    }

    #[inline]
    /// Releases ownership of the output.
    ///
    /// If you are not using a swap chain, get access to an output by calling
    /// `take_ownership` and release it when you are finished by calling
    /// `release_ownership`. An application that uses a swap chain will
    /// typically not call either of these methods.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// This method is marked as unsafe because it is not clear what the
    /// implications of calling this method are, and therefore an application
    /// should be certain this is the action they would like to take.
    ///
    /// </div>
    pub unsafe fn release_ownership(&self) {
        self.ptr.ReleaseOwnership();
    }

    #[inline]
    /// Gets a description of the gamma-control capabilities.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// Calling this method is only supported while in full-screen mode.
    ///
    /// </div>
    pub fn get_gamma_control_capabilities(&self) -> Result<GammaControlCaps, Error> {
        unsafe {
            let mut caps: GammaControlCaps = mem::uninitialized();
            let hr = self.ptr.GetGammaControlCapabilities(&mut caps.desc);
            Error::map(hr, caps)
        }
    }

    #[inline]
    /// Gets the gamma control settings.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// Calling this method is only supported while in full-screen mode.
    ///
    /// </div>
    pub fn get_gamma_control(&self) -> Result<GammaControl, Error> {
        unsafe {
            let mut control: GammaControl = mem::uninitialized();
            let hr = self.ptr.GetGammaControl(&mut control.desc);
            Error::map(hr, control)
        }
    }

    #[inline]
    /// Sets the gamma controls.
    ///
    /// For info about using gamma correction, see [Using gamma correction][1].
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// Calling this method is only supported while in full-screen mode.
    ///
    /// </div>
    ///
    /// [1]: https://msdn.microsoft.com/97ACDAE3-514E-4AAF-A27D-E5FFC162DB2A
    pub fn set_gamma_control(&self, control: &GammaControl) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetGammaControl(&control.desc);
            Error::map(hr, ())
        }
    }

    #[inline]
    /// Gets a copy of the current display surface.
    ///
    /// `get_display_surface_data` can only be called when an output is in
    /// full-screen mode. If the method succeeds, DXGI fills the destination
    /// surface.
    ///
    /// Use `get_desc` to determine the size (width and height) of the output
    /// when you want to allocate space for the destination surface. This is
    /// true regardless of target monitor rotation. A destination surface
    /// created by a graphics component (such as Direct3D 11) must be created
    /// with CPU-write permission (see [`CpuAccessFlags::WRITE`][1]). Other
    /// surfaces should be created with CPU read-write permission
    /// (see [`CpuAccessFlags::READWRITE`][2]). This
    /// method will modify the surface data to fit the destination surface
    /// (stretch, shrink, convert format, rotate). The stretch and shrink is
    /// performed with point-sampling.
    ///
    /// [1]: https://docs.rs/direct3d11/*/direct3d11/enums/struct.CpuAccessFlags.html#associatedconstant.WRITE
    /// [2]: https://docs.rs/direct3d11/*/direct3d11/enums/struct.CpuAccessFlags.html#associatedconstant.READWRITE
    pub fn get_display_surface_data(&self, surface: &Surface) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.GetDisplaySurfaceData(surface.get_raw());
            Error::map(hr, ())
        }
    }

    /// Changes the display mode. The surface must have been created as a back
    /// buffer ([`UsageFlags::BACK_BUFFER`][1]).
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// This method is marked as unsafe because it is not clear what the
    /// implications of calling this method are, and therefore an application
    /// should be certain this is the action they would like to take.
    ///
    /// </div>
    ///
    /// [1]: enums/struct.UsageFlags.html#associatedconstant.BACK_BUFFER
    pub unsafe fn set_display_surface(&self, surface: &Surface) -> Result<(), Error> {
        let hr = self.ptr.SetDisplaySurface(surface.get_raw());
        Error::map(hr, ())
    }

    #[inline]
    pub fn get_frame_statistics(&self) -> Result<FrameStatistics, Error> {
        unsafe {
            let mut stats: FrameStatistics = mem::uninitialized();
            let hr = self.ptr.GetFrameStatistics(&mut stats.desc);
            Error::map(hr, stats)
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
/// Describes an output or physical connection between the adapter (video card)
/// and a device.
pub struct OutputDesc {
    desc: DXGI_OUTPUT_DESC,
}

impl OutputDesc {
    #[inline]
    /// A string that contains the name of the output device.
    pub fn device_name(&self) -> String {
        let len = self
            .desc
            .DeviceName
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(128);
        let ostr = OsString::from_wide(&self.desc.DeviceName[..len]);
        ostr.to_string_lossy().into_owned()
    }

    #[inline]
    /// A rect containing the bounds of the output in desktop coordinates.
    /// Desktop coordinates depend on the dots per inch (DPI) of the desktop.
    /// For info about writing DPI-aware Win32 apps, see [High DPI][1].
    ///
    /// [1]: https://msdn.microsoft.com/en-us/library/Mt843498(v=VS.85).aspx
    pub fn desktop_coordinates(&self) -> Recti {
        self.desc.DesktopCoordinates.into()
    }

    #[inline]
    /// Whether the output is attached to the desktop.
    pub fn attached_to_desktop(&self) -> bool {
        self.desc.AttachedToDesktop != 0
    }

    #[inline]
    /// Describes the rotation applied to the output image.
    pub fn rotation(&self) -> UncheckedEnum<ModeRotation> {
        self.desc.Rotation.into()
    }

    #[inline]
    /// The monitor associated with this output.
    pub fn monitor(&self) -> HMONITOR {
        self.desc.Monitor
    }

    #[inline]
    pub fn raw(&self) -> &DXGI_OUTPUT_DESC {
        &self.desc
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
/// Describes a display mode.
pub struct Mode {
    desc: DXGI_MODE_DESC,
}

impl Mode {
    #[inline]
    /// Constructs a blank Mode. Zeroed, except for the refresh rate, which
    /// is initialized to `(0 / 1)`.
    pub fn new() -> Mode {
        Mode {
            desc: DXGI_MODE_DESC {
                Width: 0,
                Height: 0,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 0,
                    Denominator: 1,
                },
                Format: Format::Unknown as u32,
                ScanlineOrdering: ModeScanlineOrder::Unspecified as u32,
                Scaling: ModeScaling::Unspecified as u32,
            },
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.desc.Width
    }

    #[inline]
    pub fn set_width(&mut self, width: u32) {
        self.desc.Width = width;
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.desc.Height
    }

    #[inline]
    pub fn set_height(&mut self, height: u32) {
        self.desc.Height = height;
    }

    #[inline]
    pub fn refresh_rate(&self) -> Ratio {
        Ratio::new(
            self.desc.RefreshRate.Numerator,
            self.desc.RefreshRate.Denominator,
        )
    }

    #[inline]
    pub fn set_refresh_rate(&mut self, rate: Ratio) {
        self.desc.RefreshRate.Numerator = rate.numerator;
        self.desc.RefreshRate.Denominator = rate.denominator;
    }

    #[inline]
    pub fn format(&self) -> UncheckedEnum<Format> {
        self.desc.Format.into()
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.desc.Format = format.to_u32();
    }

    #[inline]
    pub fn scanline_ordering(&self) -> UncheckedEnum<ModeScanlineOrder> {
        self.desc.ScanlineOrdering.into()
    }

    #[inline]
    pub fn set_scanline_ordering(&mut self, ordering: ModeScanlineOrder) {
        self.desc.ScanlineOrdering = ordering as u32;
    }

    #[inline]
    pub fn scaling(&self) -> UncheckedEnum<ModeScaling> {
        self.desc.Scaling.into()
    }

    #[inline]
    pub fn set_scaling(&mut self, scaling: ModeScaling) {
        self.desc.Scaling = scaling as u32;
    }

    #[inline]
    pub fn raw(&self) -> &DXGI_MODE_DESC {
        &self.desc
    }
}

impl Default for Mode {
    #[inline]
    fn default() -> Self {
        Mode::new()
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Mode")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("refresh_rate", &self.refresh_rate())
            .field("format", &self.format())
            .field("scanline_ordering", &self.scanline_ordering())
            .field("scaling", &self.scaling())
            .finish()
    }
}

pub struct ModeBuilder {
    mode: Mode,
}

impl ModeBuilder {
    #[inline]
    pub fn new() -> ModeBuilder {
        ModeBuilder { mode: Mode::new() }
    }

    #[inline]
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.mode.set_width(width);
        self.mode.set_height(height);
        self
    }

    #[inline]
    pub fn with_refresh_rate(mut self, rate: impl Into<Ratio>) -> Self {
        self.mode.set_refresh_rate(rate.into());
        self
    }

    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.mode.set_format(format);
        self
    }

    #[inline]
    pub fn with_scanline_ordering(mut self, so: ModeScanlineOrder) -> Self {
        self.mode.set_scanline_ordering(so);
        self
    }

    #[inline]
    pub fn with_scaling(mut self, scaling: ModeScaling) -> Self {
        self.mode.set_scaling(scaling);
        self
    }

    #[inline]
    pub fn build(self) -> Mode {
        self.mode
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GammaControlCaps {
    desc: DXGI_GAMMA_CONTROL_CAPABILITIES,
}

impl GammaControlCaps {
    #[inline]
    pub fn scale_and_offset_supported(&self) -> bool {
        self.desc.ScaleAndOffsetSupported != 0
    }

    #[inline]
    pub fn max_converted_value(&self) -> f32 {
        self.desc.MaxConvertedValue
    }

    #[inline]
    pub fn min_converted_value(&self) -> f32 {
        self.desc.MinConvertedValue
    }

    #[inline]
    pub fn control_point_positions(&self) -> &[f32] {
        assert!(self.desc.NumGammaControlPoints <= 1025);
        &self.desc.ControlPointPositions[..self.desc.NumGammaControlPoints as usize]
    }
}

impl fmt::Debug for GammaControlCaps {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GammaControlCaps")
            .field(
                "scale_and_offset_supported",
                &self.scale_and_offset_supported(),
            )
            .field("max_converted_value", &self.max_converted_value())
            .field("min_converted_value", &self.min_converted_value())
            .field("control_point_positions", &self.control_point_positions())
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GammaControl {
    desc: DXGI_GAMMA_CONTROL,
}

impl GammaControl {
    #[inline]
    pub fn new() -> GammaControl {
        unsafe { mem::zeroed() }
    }

    #[inline]
    pub fn scale(&self) -> Rgb {
        unsafe { mem::transmute(self.desc.Scale) }
    }

    #[inline]
    pub fn set_scale(&mut self, scale: Rgb) {
        self.desc.Scale = scale.rgb;
    }

    #[inline]
    pub fn offset(&self) -> Rgb {
        unsafe { mem::transmute(self.desc.Offset) }
    }

    #[inline]
    pub fn set_offset(&mut self, offset: Rgb) {
        self.desc.Offset = offset.rgb;
    }

    #[inline]
    pub fn gamma_curve(&self) -> &[Rgb; 1025] {
        unsafe { mem::transmute(&self.desc.GammaCurve) }
    }

    #[inline]
    pub fn gamma_curve_mut(&mut self) -> &mut [Rgb; 1025] {
        unsafe { mem::transmute(&mut self.desc.GammaCurve) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rgb {
    rgb: DXGI_RGB,
}

impl Rgb {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb {
            rgb: DXGI_RGB {
                Red: r,
                Green: g,
                Blue: b,
            },
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.rgb.Red
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.rgb.Green
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.rgb.Blue
    }

    #[inline]
    pub fn set_r(&mut self, r: f32) {
        self.rgb.Red = r;
    }

    #[inline]
    pub fn set_g(&mut self, g: f32) {
        self.rgb.Green = g;
    }

    #[inline]
    pub fn set_b(&mut self, b: f32) {
        self.rgb.Blue = b;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rgba {
    rgba: DXGI_RGBA,
}

impl Rgba {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba {
            rgba: DXGI_RGBA { r, g, b, a },
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.rgba.r
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.rgba.g
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.rgba.b
    }

    #[inline]
    pub fn set_r(&mut self, r: f32) {
        self.rgba.r = r;
    }

    #[inline]
    pub fn set_g(&mut self, g: f32) {
        self.rgba.g = g;
    }

    #[inline]
    pub fn set_b(&mut self, b: f32) {
        self.rgba.b = b;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameStatistics {
    pub(crate) desc: DXGI_FRAME_STATISTICS,
}

impl FrameStatistics {
    #[inline]
    pub fn present_count(&self) -> u32 {
        self.desc.PresentCount
    }

    #[inline]
    pub fn present_refresh_count(&self) -> u32 {
        self.desc.PresentRefreshCount
    }

    #[inline]
    pub fn sync_refresh_count(&self) -> u32 {
        self.desc.SyncRefreshCount
    }

    #[inline]
    pub fn sync_qpc_time(&self) -> i64 {
        unsafe { *self.desc.SyncQPCTime.QuadPart() }
    }

    #[inline]
    pub fn sync_gpu_time(&self) -> i64 {
        unsafe { *self.desc.SyncGPUTime.QuadPart() }
    }
}

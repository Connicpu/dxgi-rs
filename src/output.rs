use device::Device;
use error::Error;
use surface::Surface;

use std::ffi::OsString;
use std::fmt;
use std::mem;
use std::ptr;

use num::rational::Ratio;
use winapi::shared::dxgi::{IDXGIOutput, DXGI_FRAME_STATISTICS, DXGI_OUTPUT_DESC};
use winapi::shared::dxgiformat::{DXGI_FORMAT, DXGI_FORMAT_UNKNOWN};
use winapi::shared::dxgitype::{DXGI_GAMMA_CONTROL, DXGI_GAMMA_CONTROL_CAPABILITIES,
                               DXGI_MODE_DESC, DXGI_MODE_ROTATION, DXGI_MODE_SCALING,
                               DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER,
                               DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_RATIONAL, DXGI_RGB};
use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::{HMONITOR, RECT};
use winapi::shared::winerror::{DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, S_OK};
use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;
use wio::wide::FromWide;

pub struct Output {
    ptr: ComPtr<IDXGIOutput>,
}

impl Output {
    #[inline]
    pub unsafe fn from_raw(ptr: *mut IDXGIOutput) -> Output {
        Output {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut IDXGIOutput {
        self.ptr.as_raw()
    }

    #[inline]
    pub fn get_desc(&self) -> OutputDesc {
        unsafe {
            let mut desc = mem::uninitialized();

            let result = self.ptr.GetDesc(&mut desc);
            assert!(result >= 0);

            OutputDesc { desc }
        }
    }

    #[inline]
    pub fn get_modes(&self, format: DXGI_FORMAT) -> Result<Vec<Mode>, Error> {
        unsafe {
            let mut buf: Vec<Mode> = Vec::new();
            loop {
                let mut len = 0;
                let ptr = ptr::null_mut();
                let hr = self.ptr.GetDisplayModeList(format, 2, &mut len, ptr);
                Error::map(hr, ())?;

                buf.reserve_exact(len as usize);

                let ptr = buf.as_mut_ptr() as *mut DXGI_MODE_DESC;
                let hr = self.ptr.GetDisplayModeList(format, 2, &mut len, ptr);
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
            let hr = self.ptr
                .FindClosestMatchingMode(&mode.desc, &mut matching.desc, dev);

            Error::map(hr, matching)
        }
    }

    #[inline]
    pub fn wait_for_vblank(&self) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.WaitForVBlank();
            Error::map(hr, ())
        }
    }

    #[inline]
    pub fn take_ownership(&self, device: &Device, exclusive: bool) -> Result<(), Error> {
        unsafe {
            let dev = device.get_raw();
            let hr = self.ptr.TakeOwnership(dev as *mut _, exclusive as BOOL);
            Error::map(hr, ())
        }
    }

    #[inline]
    pub fn release_ownership(&self) {
        unsafe {
            self.ptr.ReleaseOwnership();
        }
    }

    #[inline]
    pub fn get_gamma_control_capabilities(&self) -> Result<GammaControlCaps, Error> {
        unsafe {
            let mut caps: GammaControlCaps = mem::uninitialized();
            let hr = self.ptr.GetGammaControlCapabilities(&mut caps.desc);
            Error::map(hr, caps)
        }
    }

    #[inline]
    pub fn get_gamma_control(&self) -> Result<GammaControl, Error> {
        unsafe {
            let mut control: GammaControl = mem::uninitialized();
            let hr = self.ptr.GetGammaControl(&mut control.desc);
            Error::map(hr, control)
        }
    }

    #[inline]
    pub fn set_gamma_control(&self, control: &GammaControl) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetGammaControl(&control.desc);
            Error::map(hr, ())
        }
    }

    // NOTE: Windows docs say to *NEVER* use SetDisplaySurface as an application. I've omitted the
    // method for now. If someone has a use case for it, open an issue and I'll add it.

    #[inline]
    pub fn get_display_surface_data(&self, surface: &Surface) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.GetDisplaySurfaceData(surface.get_raw());
            Error::map(hr, ())
        }
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

unsafe impl Send for Output {}
unsafe impl Sync for Output {}

pub struct OutputDesc {
    desc: DXGI_OUTPUT_DESC,
}

impl OutputDesc {
    pub fn device_name(&self) -> String {
        let len = self.desc
            .DeviceName
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(128);
        let ostr = OsString::from_wide(&self.desc.DeviceName[..len]);
        ostr.to_string_lossy().into_owned()
    }

    #[inline]
    pub fn desktop_coordinates(&self) -> RECT {
        self.desc.DesktopCoordinates
    }

    #[inline]
    pub fn attached_to_desktop(&self) -> bool {
        self.desc.AttachedToDesktop != 0
    }

    #[inline]
    pub fn rotation(&self) -> DXGI_MODE_ROTATION {
        self.desc.Rotation
    }

    #[inline]
    pub fn monitor(&self) -> HMONITOR {
        self.desc.Monitor
    }
}

#[repr(C)]
pub struct Mode {
    desc: DXGI_MODE_DESC,
}

impl Mode {
    #[inline]
    pub fn new() -> Mode {
        Mode {
            desc: DXGI_MODE_DESC {
                Width: 0,
                Height: 0,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 0,
                    Denominator: 1,
                },
                Format: DXGI_FORMAT_UNKNOWN,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
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
    pub fn refresh_rate(&self) -> Ratio<u32> {
        Ratio::new(
            self.desc.RefreshRate.Numerator,
            self.desc.RefreshRate.Denominator,
        )
    }

    #[inline]
    pub fn set_refresh_rate(&mut self, rate: Ratio<u32>) {
        self.desc.RefreshRate.Numerator = *rate.numer();
        self.desc.RefreshRate.Denominator = *rate.denom();
    }

    #[inline]
    pub fn format(&self) -> DXGI_FORMAT {
        self.desc.Format
    }

    #[inline]
    pub fn set_format(&mut self, format: DXGI_FORMAT) {
        self.desc.Format = format;
    }

    #[inline]
    pub fn scanline_ordering(&self) -> DXGI_MODE_SCANLINE_ORDER {
        self.desc.ScanlineOrdering
    }

    #[inline]
    pub fn set_scanline_ordering(&mut self, ordering: DXGI_MODE_SCANLINE_ORDER) {
        self.desc.ScanlineOrdering = ordering;
    }

    #[inline]
    pub fn scaling(&self) -> DXGI_MODE_SCALING {
        self.desc.Scaling
    }

    #[inline]
    pub fn set_scaling(&mut self, scaling: DXGI_MODE_SCALING) {
        self.desc.Scaling = scaling;
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

pub struct FrameStatistics {
    desc: DXGI_FRAME_STATISTICS,
}

impl FrameStatistics {
    pub fn present_count(&self) -> u32 {
        self.desc.PresentCount
    }

    pub fn present_refresh_count(&self) -> u32 {
        self.desc.PresentRefreshCount
    }

    pub fn sync_refresh_count(&self) -> u32 {
        self.desc.SyncRefreshCount
    }

    pub fn sync_qpc_time(&self) -> i64 {
        unsafe { *self.desc.SyncQPCTime.QuadPart() }
    }

    pub fn sync_gpu_time(&self) -> i64 {
        unsafe { *self.desc.SyncGPUTime.QuadPart() }
    }
}

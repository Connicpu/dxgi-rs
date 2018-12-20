use crate::descriptions::{FrameStatistics, GammaControl, GammaControlCaps, Mode, OutputDesc};
use crate::device::Device;
use crate::enums::Format;
use dcommon::error::Error;
use crate::surface::Surface;
use winapi::shared::winerror::SUCCEEDED;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGIOutput;
use winapi::shared::dxgitype::DXGI_MODE_DESC;
use winapi::shared::minwindef::BOOL;
use winapi::shared::winerror::{DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, S_OK};
use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;

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
    pub fn desc(&self) -> OutputDesc {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.ptr.GetDesc(&mut desc);
            assert!(SUCCEEDED(hr));
            desc.into()
        }
    }

    #[inline]
    /// Gets the display modes that match the requested format and other input
    /// options.
    pub fn modes(&self, format: Format) -> Result<Vec<Mode>, Error> {
        unsafe {
            let mut buf: Vec<Mode> = Vec::new();
            loop {
                let mut len = 0;
                let ptr = std::ptr::null_mut();
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
                .unwrap_or(std::ptr::null_mut());

            let mut matching = std::mem::zeroed();
            let hr = self
                .ptr
                .FindClosestMatchingMode(&(*mode).into(), &mut matching, dev);

            Error::map(hr, matching.into())
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
    pub fn gamma_control_capabilities(&self) -> Result<GammaControlCaps, Error> {
        unsafe {
            let mut caps = std::mem::zeroed();
            let hr = self.ptr.GetGammaControlCapabilities(&mut caps);
            Error::map(hr, caps.into())
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
    pub fn gamma_control(&self) -> Result<GammaControl, Error> {
        unsafe {
            let mut control = std::mem::zeroed();
            let hr = self.ptr.GetGammaControl(&mut control);
            Error::map(hr, control.into())
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
            let hr = self.ptr.SetGammaControl(&(*control).into());
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
    pub fn frame_statistics(&self) -> Result<FrameStatistics, Error> {
        unsafe {
            let mut stats = std::mem::zeroed();
            let hr = self.ptr.GetFrameStatistics(&mut stats);
            Error::map(hr, stats.into())
        }
    }
}

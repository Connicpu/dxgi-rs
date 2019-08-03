use crate::descriptions::{FrameStatistics, GammaControl, GammaControlCaps, Mode, OutputDesc};
use crate::device::IDevice;
use crate::enums::Format;
use crate::surface::ISurface;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::IDXGIOutput;
use winapi::shared::dxgitype::DXGI_MODE_DESC;
use winapi::shared::minwindef::BOOL;
use winapi::shared::winerror::SUCCEEDED;
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

pub unsafe trait IOutput {
    /// Get a description of the output.
    fn desc(&self) -> OutputDesc {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.raw_out().GetDesc(&mut desc);
            assert!(SUCCEEDED(hr));
            desc.into()
        }
    }

    /// Gets the display modes that match the requested format and other input
    /// options.
    fn modes(&self, format: Format) -> Result<Vec<Mode>, Error> {
        unsafe {
            let mut buf: Vec<Mode> = Vec::new();
            loop {
                let mut len = 0;
                let ptr = std::ptr::null_mut();
                let hr = self
                    .raw_out()
                    .GetDisplayModeList(format as u32, 2, &mut len, ptr);
                Error::map(hr, ())?;

                buf.reserve_exact(len as usize);

                let ptr = buf.as_mut_ptr() as *mut DXGI_MODE_DESC;
                let hr = self
                    .raw_out()
                    .GetDisplayModeList(format as u32, 2, &mut len, ptr);
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

    /// Finds the display mode that most closely matches the requested display
    /// mode.
    fn find_closest_matching_mode(
        &self,
        mode: &Mode,
        device: Option<&dyn IDevice>,
    ) -> Result<Mode, Error> {
        unsafe {
            let dev: *mut IUnknown = device
                .map(|d| d.raw_dev() as *const _ as *mut _)
                .unwrap_or(std::ptr::null_mut());

            let mut matching = std::mem::zeroed();
            let hr = self
                .raw_out()
                .FindClosestMatchingMode(&(*mode).into(), &mut matching, dev);

            Error::map(hr, matching.into())
        }
    }

    /// Halt a thread until the next vertical blank occurs.
    fn wait_for_vblank(&self) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_out().WaitForVBlank();
            Error::map(hr, ())
        }
    }

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
    unsafe fn take_ownership(&self, device: &dyn IDevice, exclusive: bool) -> Result<(), Error> {
        let dev = device.raw_dev() as *const _ as *mut _;
        let hr = self.raw_out().TakeOwnership(dev, exclusive as BOOL);
        Error::map(hr, ())
    }

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
    unsafe fn release_ownership(&self) {
        self.raw_out().ReleaseOwnership();
    }

    /// Gets a description of the gamma-control capabilities.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// Calling this method is only supported while in full-screen mode.
    ///
    /// </div>
    fn gamma_control_capabilities(&self) -> Result<GammaControlCaps, Error> {
        unsafe {
            let mut caps = std::mem::zeroed();
            let hr = self.raw_out().GetGammaControlCapabilities(&mut caps);
            Error::map(hr, caps.into())
        }
    }

    /// Gets the gamma control settings.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// Calling this method is only supported while in full-screen mode.
    ///
    /// </div>
    fn gamma_control(&self) -> Result<GammaControl, Error> {
        unsafe {
            let mut control = std::mem::zeroed();
            let hr = self.raw_out().GetGammaControl(&mut control);
            Error::map(hr, control.into())
        }
    }

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
    fn set_gamma_control(&self, control: &GammaControl) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_out().SetGammaControl(&(*control).into());
            Error::map(hr, ())
        }
    }

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
    fn get_display_surface_data(&self, surface: &dyn ISurface) -> Result<(), Error> {
        unsafe {
            let hr = self
                .raw_out()
                .GetDisplaySurfaceData(surface.raw_surface() as *const _ as *mut _);
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
    unsafe fn set_display_surface(&self, surface: &dyn ISurface) -> Result<(), Error> {
        let hr = self
            .raw_out()
            .SetDisplaySurface(surface.raw_surface() as *const _ as *mut _);
        Error::map(hr, ())
    }

    fn frame_statistics(&self) -> Result<FrameStatistics, Error> {
        unsafe {
            let mut stats = std::mem::zeroed();
            let hr = self.raw_out().GetFrameStatistics(&mut stats);
            Error::map(hr, stats.into())
        }
    }

    unsafe fn raw_out(&self) -> &IDXGIOutput;
}

unsafe impl IOutput for Output {
    unsafe fn raw_out(&self) -> &IDXGIOutput {
        &self.ptr
    }
}

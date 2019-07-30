use crate::descriptions::{FullscreenDesc, PresentParameters, Rgba, SwapChainDesc1};
use crate::device::IDevice;
use crate::device_subobject::IDeviceSubObject;
use crate::enums::*;
use crate::factory::Factory2;
use crate::output::Output;
use crate::swap_chain::swap_chain::ISwapChain;
use crate::swap_chain::CoreWindowType;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::{IDXGIDeviceSubObject, IDXGISwapChain};
use winapi::shared::dxgi1_2::IDXGISwapChain1;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;
use winapi::Interface;
use wio::com::ComPtr;

pub use self::hwnd_builder::SwapChainHwndBuilder;

mod hwnd_builder;

#[derive(PartialEq, ComWrapper)]
#[com(send, debug)]
#[repr(transparent)]
pub struct SwapChain1 {
    ptr: ComPtr<IDXGISwapChain1>,
}

impl SwapChain1 {
    pub fn create_hwnd<'a>(
        factory: &'a Factory2,
        device: &'a dyn IDevice,
    ) -> SwapChainHwndBuilder<'a> {
        SwapChainHwndBuilder::create(factory, device)
    }
}

pub unsafe trait ISwapChain1: ISwapChain {
    unsafe fn raw_sc1(&self) -> &IDXGISwapChain1;

    fn desc(&self) -> SwapChainDesc1 {
        unsafe {
            let mut scd = std::mem::zeroed();
            let hr = self.raw_sc1().GetDesc1(&mut scd);
            assert!(SUCCEEDED(hr));
            scd.into()
        }
    }

    fn fullscreen_desc(&self) -> FullscreenDesc {
        unsafe {
            let mut fd = std::mem::zeroed();
            let hr = self.raw_sc1().GetFullscreenDesc(&mut fd);
            assert!(SUCCEEDED(hr));
            fd.into()
        }
    }

    fn hwnd(&self) -> Option<HWND> {
        unsafe {
            let mut hwnd = std::ptr::null_mut();
            let hr = self.raw_sc1().GetHwnd(&mut hwnd);
            if SUCCEEDED(hr) {
                Some(hwnd)
            } else {
                None
            }
        }
    }

    fn core_window<W: CoreWindowType>(&self) -> Option<W>
    where
        Self: Sized,
    {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self
                .raw_sc1()
                .GetCoreWindow(&W::Interface::uuidof(), &mut ptr);
            if SUCCEEDED(hr) {
                Some(W::from_raw(ptr as _))
            } else {
                None
            }
        }
    }

    fn present1(
        &mut self,
        sync_interval: u32,
        flags: PresentFlags,
        parameters: &PresentParameters,
    ) -> Result<(), Error> {
        unsafe {
            let hr = self
                .raw_sc1()
                .Present1(sync_interval, flags.0, &parameters.into());
            Error::map(hr, ())
        }
    }

    fn is_temporary_mono_supported(&self) -> bool {
        unsafe { self.raw_sc1().IsTemporaryMonoSupported() != 0 }
    }

    fn restrict_to_output(&self) -> Option<Output> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_sc1().GetRestrictToOutput(&mut ptr);
            if SUCCEEDED(hr) {
                Some(Output::from_raw(ptr))
            } else {
                None
            }
        }
    }

    fn set_background_color(&mut self, bg: &Rgba) -> Result<(), Error> {
        unsafe {
            let hr = self
                .raw_sc1()
                .SetBackgroundColor(bg as *const _ as *const _);
            Error::map(hr, ())
        }
    }

    fn background_color(&self) -> Option<Rgba> {
        unsafe {
            let mut color = std::mem::zeroed();
            let hr = self.raw_sc1().GetBackgroundColor(&mut color);
            if SUCCEEDED(hr) {
                Some(color.into())
            } else {
                None
            }
        }
    }

    fn set_rotation(&mut self, rot: ModeRotation) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_sc1().SetRotation(rot as u32);
            Error::map(hr, ())
        }
    }

    fn rotation(&self) -> Result<ModeRotation, Error> {
        unsafe {
            let mut rot = 0;
            let hr = self.raw_sc1().GetRotation(&mut rot);
            let rot = ModeRotation::from_u32(rot).unwrap_or(ModeRotation::Unspecified);
            Error::map(hr, rot)
        }
    }
}

unsafe impl IDeviceSubObject for SwapChain1 {
    unsafe fn raw_dso(&self) -> &IDXGIDeviceSubObject {
        &self.ptr
    }
}

unsafe impl ISwapChain for SwapChain1 {
    unsafe fn raw_sc(&self) -> &IDXGISwapChain {
        &self.ptr
    }
}

unsafe impl ISwapChain1 for SwapChain1 {
    unsafe fn raw_sc1(&self) -> &IDXGISwapChain1 {
        &self.ptr
    }
}

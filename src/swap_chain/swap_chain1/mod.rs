use crate::descriptions::{FullscreenDesc, PresentParameters, Rgba, SwapChainDesc1};
use crate::device::Device;
use crate::enums::*;
use crate::error::Error;
use crate::factory::Factory2;
use crate::helpers::{deref_com_wrapper, deref_com_wrapper_mut};
use crate::output::Output;
use crate::swap_chain::swap_chain::SwapChain;
use crate::swap_chain::CoreWindowType;
use crate::swap_chain::SwapChainType;

use com_wrapper::ComWrapper;
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
    pub fn create_hwnd<'a>(factory: &'a Factory2, device: &'a Device) -> SwapChainHwndBuilder<'a> {
        SwapChainHwndBuilder::create(factory, device)
    }

    pub fn desc(&self) -> SwapChainDesc1 {
        unsafe {
            let mut scd = std::mem::zeroed();
            let hr = self.ptr.GetDesc1(&mut scd);
            assert!(SUCCEEDED(hr));
            scd.into()
        }
    }

    pub fn fullscreen_desc(&self) -> FullscreenDesc {
        unsafe {
            let mut fd = std::mem::zeroed();
            let hr = self.ptr.GetFullscreenDesc(&mut fd);
            assert!(SUCCEEDED(hr));
            fd.into()
        }
    }

    pub fn hwnd(&self) -> Option<HWND> {
        unsafe {
            let mut hwnd = std::ptr::null_mut();
            let hr = self.ptr.GetHwnd(&mut hwnd);
            if SUCCEEDED(hr) {
                Some(hwnd)
            } else {
                None
            }
        }
    }

    pub fn core_window<W: CoreWindowType>(&self) -> Option<W> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetCoreWindow(&W::Interface::uuidof(), &mut ptr);
            if SUCCEEDED(hr) {
                Some(W::from_raw(ptr as _))
            } else {
                None
            }
        }
    }

    pub fn present1(
        &mut self,
        sync_interval: u32,
        flags: PresentFlags,
        parameters: &PresentParameters,
    ) -> Result<(), Error> {
        unsafe {
            let hr = self
                .ptr
                .Present1(sync_interval, flags.0, &parameters.into());
            Error::map(hr, ())
        }
    }

    pub fn is_temporary_mono_supported(&self) -> bool {
        unsafe { self.ptr.IsTemporaryMonoSupported() != 0 }
    }

    pub fn restrict_to_output(&self) -> Option<Output> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetRestrictToOutput(&mut ptr);
            if SUCCEEDED(hr) {
                Some(Output::from_raw(ptr))
            } else {
                None
            }
        }
    }

    pub fn set_background_color(&mut self, bg: &Rgba) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetBackgroundColor(bg as *const _ as *const _);
            Error::map(hr, ())
        }
    }

    pub fn background_color(&self) -> Option<Rgba> {
        unsafe {
            let mut color = std::mem::zeroed();
            let hr = self.ptr.GetBackgroundColor(&mut color);
            if SUCCEEDED(hr) {
                Some(color.into())
            } else {
                None
            }
        }
    }

    pub fn set_rotation(&mut self, rot: ModeRotation) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetRotation(rot as u32);
            Error::map(hr, ())
        }
    }

    pub fn rotation(&self) -> Result<ModeRotation, Error> {
        unsafe {
            let mut rot = 0;
            let hr = self.ptr.GetRotation(&mut rot);
            let rot = ModeRotation::from_u32(rot).unwrap_or(ModeRotation::Unspecified);
            Error::map(hr, rot)
        }
    }
}

impl SwapChainType for SwapChain1 {}

impl std::ops::Deref for SwapChain1 {
    type Target = SwapChain;
    fn deref(&self) -> &SwapChain {
        unsafe { deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for SwapChain1 {
    fn deref_mut(&mut self) -> &mut SwapChain {
        unsafe { deref_com_wrapper_mut(self) }
    }
}

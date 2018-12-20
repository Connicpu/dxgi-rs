use crate::descriptions::{FrameStatistics, Mode, SwapChainDesc};
use crate::device_subobject::DeviceSubObject;
use crate::enums::*;
use dcommon::error::Error;
use crate::helpers::{deref_com_wrapper, deref_com_wrapper_mut};
use crate::output::Output;
use crate::swap_chain::resize_buffers::ResizeBuffers;
use crate::swap_chain::BackbufferTexture;
use crate::swap_chain::FullscreenState;
use crate::swap_chain::SwapChainType;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGISwapChain;
use winapi::shared::winerror::{SUCCEEDED, S_OK};
use winapi::Interface;
use wio::com::ComPtr;

#[derive(PartialEq, ComWrapper)]
#[com(send, debug)]
#[repr(transparent)]
pub struct SwapChain {
    ptr: ComPtr<IDXGISwapChain>,
}

impl SwapChain {
    pub fn desc(&self) -> SwapChainDesc {
        unsafe {
            let mut scd = std::mem::zeroed();
            let hr = self.ptr.GetDesc(&mut scd);
            assert!(SUCCEEDED(hr));
            scd.into()
        }
    }

    pub fn present(&mut self, sync_interval: u32, flags: PresentFlags) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.Present(sync_interval, flags.0);
            Error::map(hr, ())
        }
    }

    pub fn buffer<B>(&self, buffer: u32) -> Result<B, Error>
    where
        B: BackbufferTexture,
    {
        unsafe {
            let uuid = B::Interface::uuidof();
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetBuffer(buffer, &uuid, &mut ptr);
            Error::map_if(hr, || B::from_raw(ptr as _))
        }
    }

    pub fn set_fullscreen_state(&mut self, state: FullscreenState) -> Result<(), Error> {
        unsafe {
            let (fullscreen, out) = match &state {
                FullscreenState::Windowed => (false, std::ptr::null_mut()),
                FullscreenState::Fullscreen(None) => (true, std::ptr::null_mut()),
                FullscreenState::Fullscreen(Some(out)) => (true, out.get_raw()),
            };

            let hr = self.ptr.SetFullscreenState(fullscreen as i32, out);
            Error::map(hr, ())
        }
    }

    pub fn fullscreen_state(&self) -> Result<FullscreenState, Error> {
        unsafe {
            let mut isfs = 0;
            let mut out = std::ptr::null_mut();
            let hr = self.ptr.GetFullscreenState(&mut isfs, &mut out);

            if SUCCEEDED(hr) {
                let out = if out.is_null() {
                    None
                } else {
                    Some(Output::from_raw(out))
                };

                if isfs != 0 {
                    Ok(FullscreenState::Fullscreen(out))
                } else {
                    Ok(FullscreenState::Windowed)
                }
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn resize_buffers(&mut self) -> ResizeBuffers {
        let desc = self.desc();
        ResizeBuffers {
            swap_chain: &self.ptr,
            count: desc.buffer_count,
            width: desc.buffer_desc.width,
            height: desc.buffer_desc.height,
            format: desc.buffer_desc.format,
            flags: desc.flags,
        }
    }

    pub fn resize_target(&mut self, mode: &Mode) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.ResizeTarget(&(*mode).into());
            Error::map(hr, ())
        }
    }

    pub fn containing_output(&self) -> Option<Output> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetContainingOutput(&mut ptr);
            if hr == S_OK {
                Some(Output::from_raw(ptr))
            } else {
                None
            }
        }
    }

    pub fn frame_statistics(&self) -> Result<FrameStatistics, Error> {
        unsafe {
            let mut fs = std::mem::zeroed();
            let hr = self.ptr.GetFrameStatistics(&mut fs);
            Error::map(hr, fs.into())
        }
    }

    pub fn last_present_count(&self) -> Option<u32> {
        unsafe {
            let mut count = 0;
            let hr = self.ptr.GetLastPresentCount(&mut count);
            if hr == S_OK {
                Some(count)
            } else {
                None
            }
        }
    }
}

impl SwapChainType for SwapChain {}

impl std::ops::Deref for SwapChain {
    type Target = DeviceSubObject;
    fn deref(&self) -> &DeviceSubObject {
        unsafe { deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for SwapChain {
    fn deref_mut(&mut self) -> &mut DeviceSubObject {
        unsafe { deref_com_wrapper_mut(self) }
    }
}

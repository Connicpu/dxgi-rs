use crate::descriptions::{FrameStatistics, Mode, SwapChainDesc};
use crate::enums::*;
use crate::output::Output;
use crate::swap_chain::resize_buffers::ResizeBuffers;
use crate::swap_chain::BackbufferTexture;
use crate::swap_chain::FullscreenState;
use crate::device_subobject::IDeviceSubObject;

use dcommon::error::Error;
use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGISwapChain, IDXGIDeviceSubObject};
use winapi::shared::winerror::{SUCCEEDED, S_OK};
use winapi::Interface;
use wio::com::ComPtr;

#[derive(PartialEq, ComWrapper)]
#[com(send, debug)]
#[repr(transparent)]
pub struct SwapChain {
    ptr: ComPtr<IDXGISwapChain>,
}

pub unsafe trait ISwapChain: IDeviceSubObject {
    unsafe fn raw_sc(&self) -> &IDXGISwapChain;

    fn desc(&self) -> SwapChainDesc {
        unsafe {
            let mut scd = std::mem::zeroed();
            let hr = self.raw_sc().GetDesc(&mut scd);
            assert!(SUCCEEDED(hr));
            scd.into()
        }
    }

    fn present(&mut self, sync_interval: u32, flags: PresentFlags) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_sc().Present(sync_interval, flags.0);
            Error::map(hr, ())
        }
    }

    fn buffer<B>(&self, buffer: u32) -> Result<B, Error>
    where
        B: BackbufferTexture,
    {
        unsafe {
            let uuid = B::Interface::uuidof();
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_sc().GetBuffer(buffer, &uuid, &mut ptr);
            Error::map_if(hr, || B::from_raw(ptr as _))
        }
    }

    fn set_fullscreen_state(&mut self, state: FullscreenState) -> Result<(), Error> {
        unsafe {
            let (fullscreen, out) = match &state {
                FullscreenState::Windowed => (false, std::ptr::null_mut()),
                FullscreenState::Fullscreen(None) => (true, std::ptr::null_mut()),
                FullscreenState::Fullscreen(Some(out)) => (true, out.get_raw()),
            };

            let hr = self.raw_sc().SetFullscreenState(fullscreen as i32, out);
            Error::map(hr, ())
        }
    }

    fn fullscreen_state(&self) -> Result<FullscreenState, Error> {
        unsafe {
            let mut isfs = 0;
            let mut out = std::ptr::null_mut();
            let hr = self.raw_sc().GetFullscreenState(&mut isfs, &mut out);

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

    fn resize_buffers(&mut self) -> ResizeBuffers {
        let desc = self.desc();
        ResizeBuffers {
            swap_chain: unsafe { self.raw_sc() },
            count: desc.buffer_count,
            width: desc.buffer_desc.width,
            height: desc.buffer_desc.height,
            format: desc.buffer_desc.format,
            flags: desc.flags,
        }
    }

    fn resize_target(&mut self, mode: &Mode) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_sc().ResizeTarget(&(*mode).into());
            Error::map(hr, ())
        }
    }

    fn containing_output(&self) -> Option<Output> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_sc().GetContainingOutput(&mut ptr);
            if hr == S_OK {
                Some(Output::from_raw(ptr))
            } else {
                None
            }
        }
    }

    fn frame_statistics(&self) -> Result<FrameStatistics, Error> {
        unsafe {
            let mut fs = std::mem::zeroed();
            let hr = self.raw_sc().GetFrameStatistics(&mut fs);
            Error::map(hr, fs.into())
        }
    }

    fn last_present_count(&self) -> Option<u32> {
        unsafe {
            let mut count = 0;
            let hr = self.raw_sc().GetLastPresentCount(&mut count);
            if hr == S_OK {
                Some(count)
            } else {
                None
            }
        }
    }
}

unsafe impl IDeviceSubObject for SwapChain {
    unsafe fn raw_dso(&self) -> &IDXGIDeviceSubObject {
        &self.ptr
    }
}

unsafe impl ISwapChain for SwapChain {
    unsafe fn raw_sc(&self) -> &IDXGISwapChain {
        &self.ptr
    }
}

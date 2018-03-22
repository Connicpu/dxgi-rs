use device::Device;
use error::Error;
use factory::Factory;
use output::Output;

use std::mem;
use std::ptr;

use boolinator::Boolinator;
use num::rational::Ratio;
use winapi::ctypes::c_void;
use winapi::shared::dxgiformat::DXGI_FORMAT;
use winapi::shared::dxgitype::{DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_USAGE};
use winapi::shared::dxgi::{DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_EFFECT};
use winapi::shared::dxgi1_2::{DXGI_SWAP_CHAIN_DESC1, IDXGISwapChain1, DXGI_ALPHA_MODE,
                              DXGI_SCALING, DXGI_SWAP_CHAIN_FULLSCREEN_DESC};
use winapi::shared::guiddef::GUID;
use winapi::shared::windef::HWND;
use wio::com::ComPtr;

pub struct SwapChain {
    ptr: ComPtr<IDXGISwapChain1>,
}

impl SwapChain {
    #[inline]
    pub unsafe fn from_raw(ptr: *mut IDXGISwapChain1) -> SwapChain {
        SwapChain {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut IDXGISwapChain1 {
        self.ptr.as_raw()
    }

    #[inline]
    pub fn present(&self, sync_interval: u32, flags: u32) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.Present(sync_interval, flags);
            Error::map(hr, ())
        }
    }

    #[inline]
    pub fn get_buffer<B>(&self, buffer: u32) -> Result<B, Error>
    where
        B: BackbufferTexture,
    {
        unsafe {
            let uuid = B::uuidof();
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetBuffer(buffer, &uuid, &mut ptr);
            Error::map_if(hr, || B::from_raw(ptr))
        }
    }

    #[inline]
    pub fn get_fullscreen_state(&self) -> Result<Option<Output>, Error> {
        unsafe {
            let mut isfs = 0;
            let mut out = ptr::null_mut();
            let hr = self.ptr.GetFullscreenState(&mut isfs, &mut out);
            let isfs = isfs != 0;

            Error::map_if(hr, || isfs.as_some_from(|| Output::from_raw(out)))
        }
    }

    // TODO: get_desc
    // TODO: get_fullscreen_desc
    // TODO: get_hwnd
    // TODO: get_core_window
    // TODO: get_restrict_to_output
    // TODO: get_containing_output
    // TODO: get_frame_statistics
    // TODO: get_last_present_count
    // TODO: get_background_color
    // TODO: get_rotation

    // TODO: set_background_color
    // TODO: set_rotation
    // TODO: set_fullscreen_state
    // TODO: resize_buffers
    // TODO: resize_target
}

unsafe impl Send for SwapChain {}
unsafe impl Sync for SwapChain {}

/// This should be implemented for e.g. d3d11::Texture2d
pub unsafe trait BackbufferTexture {
    fn uuidof() -> GUID;
    fn from_raw(raw: *mut c_void) -> Self;
}

pub struct SwapChainHwndBuilder<'a> {
    factory: &'a Factory,
    device: &'a Device,
    hwnd: HWND,
    desc: DXGI_SWAP_CHAIN_DESC1,
    fs_desc: DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    restrict_out: Option<&'a Output>,
}

impl<'a> SwapChainHwndBuilder<'a> {
    #[inline]
    pub(crate) fn create(factory: &'a Factory, device: &'a Device) -> Self {
        SwapChainHwndBuilder {
            factory,
            device,
            hwnd: ptr::null_mut(),
            desc: def_swapchain_desc(),
            fs_desc: def_fs_swapchain_desc(),
            restrict_out: None,
        }
    }

    pub fn build(self) -> Result<SwapChain, Error> {
        unsafe {
            let factory = self.factory.get_raw();
            let mut ptr = ptr::null_mut();
            let hr = (*factory).CreateSwapChainForHwnd(
                self.device.get_raw() as *mut _,
                self.hwnd,
                &self.desc,
                &self.fs_desc,
                self.restrict_out
                    .map(|o| o.get_raw())
                    .unwrap_or(ptr::null_mut()),
                &mut ptr,
            );

            Error::map_if(hr, || SwapChain::from_raw(ptr))
        }
    }

    #[inline]
    /// Required
    pub fn hwnd(mut self, hwnd: HWND) -> Self {
        self.hwnd = hwnd;
        self
    }

    #[inline]
    /// Optional
    pub fn restrict_output(mut self, out: &'a Output) -> Self {
        self.restrict_out = Some(out);
        self
    }
}

fn def_swapchain_desc() -> DXGI_SWAP_CHAIN_DESC1 {
    use winapi::shared::dxgiformat::*;
    use winapi::shared::dxgitype::*;

    let mut desc: DXGI_SWAP_CHAIN_DESC1 = unsafe { mem::zeroed() };
    desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    desc.SampleDesc = DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0,
    };
    desc.BufferUsage = DXGI_USAGE_BACK_BUFFER | DXGI_USAGE_RENDER_TARGET_OUTPUT;
    desc.BufferCount = 2;

    desc
}

fn def_fs_swapchain_desc() -> DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    let mut desc: DXGI_SWAP_CHAIN_FULLSCREEN_DESC = unsafe { mem::zeroed() };

    desc.RefreshRate.Numerator = 60;
    desc.RefreshRate.Denominator = 1;
    desc.Windowed = 1;

    desc
}

macro_rules! impl_scbuilder_desc_fns {
    ($builder:ident) => {
        impl<'a> $builder <'a> {
            #[inline]
            /// Default is 0x0 (i.e. auto-detect)
            pub fn size(mut self, width: u32, height: u32) -> Self {
                self.desc.Width = width;
                self.desc.Height = height;
                self
            }

            #[inline]
            /// Default RGBA8 UNORM
            pub fn format(mut self, format: DXGI_FORMAT) -> Self {
                self.desc.Format = format;
                self
            }

            #[inline]
            /// Enable MSAA. Default is 1, 0
            pub fn samples(mut self, count: u32, quality: u32) -> Self {
                self.desc.SampleDesc.Count = count;
                self.desc.SampleDesc.Quality = quality;
                self
            }

            #[inline]
            /// Default is BACK_BUFFER | RENDER_TARGET_OUTPUT
            pub fn buffer_usage(mut self, usage: DXGI_USAGE) -> Self {
                self.desc.BufferUsage = usage;
                self
            }

            #[inline]
            /// Default is 2
            pub fn buffer_count(mut self, count: u32) -> Self {
                self.desc.BufferCount = count;
                self
            }

            #[inline]
            /// Default is STRETCH
            pub fn scaling(mut self, scaling: DXGI_SCALING) -> Self {
                self.desc.Scaling = scaling;
                self
            }

            #[inline]
            /// Default is DISCARD
            pub fn swap_effect(mut self, effect: DXGI_SWAP_EFFECT) -> Self {
                self.desc.SwapEffect = effect;
                self
            }

            #[inline]
            /// Default is UNSPECIFIED
            pub fn alpha_mode(mut self, mode: DXGI_ALPHA_MODE) -> Self {
                self.desc.AlphaMode = mode;
                self
            }

            #[inline]
            /// None specified by default
            pub fn flags(mut self, flags: DXGI_SWAP_CHAIN_FLAG) -> Self {
                self.desc.Flags = flags;
                self
            }

            #[inline]
            /// Default is 60/1
            pub fn refresh_rate(mut self, hz: Ratio<u32>) -> Self {
                self.fs_desc.RefreshRate.Numerator = *hz.numer();
                self.fs_desc.RefreshRate.Denominator = *hz.denom();
                self
            }

            #[inline]
            /// Default is UNSPECIFIED
            pub fn scanline_ordering(mut self, order: DXGI_MODE_SCANLINE_ORDER) -> Self {
                self.fs_desc.ScanlineOrdering = order;
                self
            }

            #[inline]
            /// Default is UNSPECIFIED
            pub fn fullscreen_scaling(mut self, scaling: DXGI_MODE_SCALING) -> Self {
                self.fs_desc.Scaling = scaling;
                self
            }

            #[inline]
            /// Default is true
            pub fn windowed(mut self, windowed: bool) -> Self {
                self.fs_desc.Windowed = if windowed { 1 } else { 0 };
                self
            }
        }
    };
}

impl_scbuilder_desc_fns!(SwapChainHwndBuilder);

use device::Device;
use error::Error;
use factory::Factory;
use output::{Mode, Output};

use std::mem;
use std::ptr;

use boolinator::Boolinator;
use num::rational::Ratio;
use winapi::ctypes::c_void;
use winapi::shared::dxgiformat::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT};
use winapi::shared::dxgitype::{DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,
                               DXGI_SAMPLE_DESC, DXGI_USAGE, DXGI_USAGE_BACK_BUFFER,
                               DXGI_USAGE_RENDER_TARGET_OUTPUT};
use winapi::shared::dxgi::{DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_EFFECT};
use winapi::shared::dxgi1_2::{DXGI_SWAP_CHAIN_DESC1, IDXGISwapChain1, DXGI_ALPHA_MODE,
                              DXGI_SCALING, DXGI_SWAP_CHAIN_FULLSCREEN_DESC};
use winapi::shared::guiddef::GUID;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;
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

    #[inline]
    pub fn get_desc(&self) -> SwapChainDesc {
        unsafe {
            let mut scd: SwapChainDesc = mem::uninitialized();
            let hr = self.ptr.GetDesc1(&mut scd.desc);
            assert!(SUCCEEDED(hr));
            scd
        }
    }

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

    #[inline]
    pub fn resize_buffers(&self) -> ResizeBuffers {
        let desc = self.get_desc();
        ResizeBuffers {
            swap_chain: &self.ptr,
            count: desc.buffer_count(),
            width: desc.width(),
            height: desc.height(),
            format: desc.format(),
            flags: desc.flags(),
        }
    }

    #[inline]
    pub fn resize_target(&self, mode: &Mode) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.ResizeTarget(mode.raw());
            Error::map(hr, ())
        }
    }
}

unsafe impl Send for SwapChain {}
unsafe impl Sync for SwapChain {}

/// This should be implemented for e.g. d3d11::Texture2d
pub unsafe trait BackbufferTexture {
    fn uuidof() -> GUID;
    fn from_raw(raw: *mut c_void) -> Self;
}

pub struct SwapChainDesc {
    desc: DXGI_SWAP_CHAIN_DESC1,
}

impl SwapChainDesc {
    #[inline]
    pub fn width(&self) -> u32 {
        self.desc.Width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.desc.Height
    }

    #[inline]
    pub fn format(&self) -> DXGI_FORMAT {
        self.desc.Format
    }

    #[inline]
    pub fn stereo(&self) -> bool {
        self.desc.Stereo != 0
    }

    #[inline]
    pub fn sample_count(&self) -> u32 {
        self.desc.SampleDesc.Count
    }

    #[inline]
    pub fn sample_quality(&self) -> u32 {
        self.desc.SampleDesc.Quality
    }

    #[inline]
    pub fn buffer_usage(&self) -> DXGI_USAGE {
        self.desc.BufferUsage
    }

    #[inline]
    pub fn buffer_count(&self) -> u32 {
        self.desc.BufferCount
    }

    #[inline]
    pub fn scaling(&self) -> DXGI_SCALING {
        self.desc.Scaling
    }

    #[inline]
    pub fn swap_effect(&self) -> DXGI_SWAP_EFFECT {
        self.desc.SwapEffect
    }

    #[inline]
    pub fn alpha_mode(&self) -> DXGI_ALPHA_MODE {
        self.desc.AlphaMode
    }

    #[inline]
    pub fn flags(&self) -> DXGI_SWAP_CHAIN_FLAG {
        self.desc.Flags
    }
}

#[derive(Copy, Clone)]
pub struct FullscreenDesc {
    desc: DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
}

impl FullscreenDesc {
    #[inline]
    pub fn refresh_rate(&self) -> Ratio<u32> {
        Ratio::new(self.desc.RefreshRate.Numerator, self.desc.RefreshRate.Denominator)
    }

    #[inline]
    pub fn scanline_ordering(&self) -> DXGI_MODE_SCANLINE_ORDER {
        self.desc.ScanlineOrdering
    }

    #[inline]
    pub fn scaling(&self) -> DXGI_MODE_SCALING {
        self.desc.Scaling
    }

    #[inline]
    pub fn windowed(&self) -> bool {
        self.desc.Windowed != 0
    }
}

#[must_use]
pub struct ResizeBuffers<'a> {
    swap_chain: &'a IDXGISwapChain1,
    count: u32,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
    flags: DXGI_SWAP_CHAIN_FLAG,
}

impl<'a> ResizeBuffers<'a> {
    #[inline]
    pub fn finish(self) -> Result<(), Error> {
        unsafe {
            let hr = self.swap_chain.ResizeBuffers(
                self.count,
                self.width,
                self.height,
                self.format,
                self.flags,
            );

            Error::map(hr, ())
        }
    }

    #[inline]
    pub fn dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    #[inline]
    pub fn format(mut self, format: DXGI_FORMAT) -> Self {
        self.format = format;
        self
    }

    #[inline]
    pub fn buffer_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: DXGI_SWAP_CHAIN_FLAG) -> Self {
        self.flags = flags;
        self
    }

    #[inline]
    pub fn modify_flags<F>(mut self, func: F) -> Self
    where
        F: FnOnce(DXGI_SWAP_CHAIN_FLAG) -> DXGI_SWAP_CHAIN_FLAG,
    {
        self.flags = func(self.flags);
        self
    }
}

#[must_use]
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
            desc: DEF_DESC,
            fs_desc: DEF_FS_DESC,
            restrict_out: None,
        }
    }

    #[inline]
    pub fn build(self) -> Result<SwapChain, Error> {
        assert!(!self.hwnd.is_null());
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

const DEF_DESC: DXGI_SWAP_CHAIN_DESC1 = DXGI_SWAP_CHAIN_DESC1 {
    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
    SampleDesc: DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0,
    },
    BufferUsage: DXGI_USAGE_BACK_BUFFER | DXGI_USAGE_RENDER_TARGET_OUTPUT,
    BufferCount: 2,
    AlphaMode: 0,
    Flags: 0,
    Width: 0,
    Height: 0,
    Scaling: 0,
    Stereo: 0,
    SwapEffect: 0,
};

const DEF_FS_DESC: DXGI_SWAP_CHAIN_FULLSCREEN_DESC = DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    RefreshRate: DXGI_RATIONAL {
        Numerator: 60,
        Denominator: 1,
    },
    Windowed: 1,
    Scaling: 0,
    ScanlineOrdering: 0,
};

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

use device::Device;
use error::Error;
use factory::Factory;
use flags::{AlphaMode, Format, ModeRotation, ModeScaling, ModeScanlineOrder, PresentFlags,
            Scaling, SwapChainFlags, SwapEffect, UsageFlags};
use output::{FrameStatistics, Mode, Output, Rgba};

use std::mem;
use std::ptr;

use boolinator::Boolinator;
use num::rational::Ratio;
use winapi::ctypes::c_void;
use winapi::shared::dxgi::DXGI_SWAP_EFFECT;
use winapi::shared::dxgi1_2::{DXGI_SWAP_CHAIN_DESC1, IDXGISwapChain1,
                              DXGI_SWAP_CHAIN_FULLSCREEN_DESC};
use winapi::shared::dxgitype::{DXGI_RATIONAL, DXGI_SAMPLE_DESC};
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[derive(Clone, PartialEq)]
pub struct SwapChain {
    ptr: ComPtr<IDXGISwapChain1>,
}

impl SwapChain {
    #[inline]
    pub fn create_hwnd<'a>(factory: &'a Factory, device: &'a Device) -> SwapChainHwndBuilder<'a> {
        SwapChainHwndBuilder::create(factory, device)
    }

    #[inline]
    pub fn present(&self, sync_interval: u32, flags: PresentFlags) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.Present(sync_interval, flags.0);
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

    #[inline]
    pub fn get_fullscreen_desc(&self) -> FullscreenDesc {
        unsafe {
            let mut fd: FullscreenDesc = mem::uninitialized();
            let hr = self.ptr.GetFullscreenDesc(&mut fd.desc);
            assert!(SUCCEEDED(hr));
            fd
        }
    }

    #[inline]
    pub fn get_hwnd(&self) -> Option<HWND> {
        unsafe {
            let mut hwnd = ptr::null_mut();
            let hr = self.ptr.GetHwnd(&mut hwnd);
            (hr == 0).as_some(hwnd)
        }
    }

    #[inline]
    pub fn get_restrict_to_output(&self) -> Option<Output> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetRestrictToOutput(&mut ptr);
            (hr == 0).as_some_from(|| Output::from_raw(ptr))
        }
    }

    #[inline]
    pub fn get_containing_output(&self) -> Option<Output> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetContainingOutput(&mut ptr);
            (hr == 0).as_some_from(|| Output::from_raw(ptr))
        }
    }

    #[inline]
    pub fn get_frame_statistics(&self) -> Result<FrameStatistics, Error> {
        unsafe {
            let mut fs: FrameStatistics = mem::uninitialized();
            let hr = self.ptr.GetFrameStatistics(&mut fs.desc);
            Error::map(hr, fs)
        }
    }

    #[inline]
    pub fn get_last_present_count(&self) -> Option<u32> {
        unsafe {
            let mut count = 0;
            let hr = self.ptr.GetLastPresentCount(&mut count);
            (hr == 0).as_some(count)
        }
    }

    #[inline]
    pub fn get_background_color(&self) -> Option<Rgba> {
        unsafe {
            let mut color = mem::uninitialized();
            let hr = self.ptr.GetBackgroundColor(&mut color);
            (hr == 0).as_some_from(|| Rgba::new(color.r, color.g, color.b, color.a))
        }
    }

    #[inline]
    pub fn get_rotation(&self) -> Result<ModeRotation, Error> {
        unsafe {
            let mut rot = 0;
            let hr = self.ptr.GetRotation(&mut rot);
            let rot = ModeRotation::try_from(rot).unwrap_or(ModeRotation::Unspecified);
            Error::map(hr, rot)
        }
    }

    #[inline]
    pub fn set_background_color(&self, bg: &Rgba) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetBackgroundColor(bg as *const _ as *const _);
            Error::map(hr, ())
        }
    }

    #[inline]
    pub fn set_rotation(&self, rot: ModeRotation) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetRotation(rot as u32);
            Error::map(hr, ())
        }
    }

    #[inline]
    pub fn set_fullscreen_state(
        &self,
        fullscreen: bool,
        output: Option<&Output>,
    ) -> Result<(), Error> {
        unsafe {
            let out = fullscreen
                .and_option(output)
                .map(|o| o.get_raw())
                .unwrap_or(ptr::null_mut());

            let hr = self.ptr.SetFullscreenState(fullscreen as BOOL, out);
            Error::map(hr, ())
        }
    }

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
}

unsafe impl Send for SwapChain {}
unsafe impl Sync for SwapChain {}

/// This should be implemented for e.g. d3d11::Texture2d
pub unsafe trait BackbufferTexture {
    fn uuidof() -> GUID;
    unsafe fn from_raw(raw: *mut c_void) -> Self;
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
    pub fn format(&self) -> Format {
        Format::try_from(self.desc.Format).unwrap_or(Format::Unknown)
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
    pub fn buffer_usage(&self) -> UsageFlags {
        UsageFlags(self.desc.BufferUsage)
    }

    #[inline]
    pub fn buffer_count(&self) -> u32 {
        self.desc.BufferCount
    }

    #[inline]
    pub fn scaling(&self) -> Scaling {
        Scaling::try_from(self.desc.Scaling).unwrap_or(Scaling::Stretch)
    }

    #[inline]
    pub fn swap_effect(&self) -> SwapEffect {
        SwapEffect::try_from(self.desc.SwapEffect).unwrap_or(SwapEffect::Discard)
    }

    #[inline]
    pub fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::try_from(self.desc.AlphaMode).unwrap_or(AlphaMode::Unspecified)
    }

    #[inline]
    pub fn flags(&self) -> SwapChainFlags {
        SwapChainFlags(self.desc.Flags)
    }
}

#[derive(Copy, Clone)]
pub struct FullscreenDesc {
    desc: DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
}

impl FullscreenDesc {
    #[inline]
    pub fn refresh_rate(&self) -> Ratio<u32> {
        Ratio::new(
            self.desc.RefreshRate.Numerator,
            self.desc.RefreshRate.Denominator,
        )
    }

    #[inline]
    pub fn scanline_ordering(&self) -> ModeScanlineOrder {
        ModeScanlineOrder::try_from(self.desc.ScanlineOrdering)
            .unwrap_or(ModeScanlineOrder::Unspecified)
    }

    #[inline]
    pub fn scaling(&self) -> ModeScaling {
        ModeScaling::try_from(self.desc.Scaling).unwrap_or(ModeScaling::Unspecified)
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
    format: Format,
    flags: SwapChainFlags,
}

impl<'a> ResizeBuffers<'a> {
    #[inline]
    pub fn finish(self) -> Result<(), Error> {
        unsafe {
            let hr = self.swap_chain.ResizeBuffers(
                self.count,
                self.width,
                self.height,
                self.format as u32,
                self.flags.0,
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
    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    #[inline]
    pub fn buffer_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: SwapChainFlags) -> Self {
        self.flags = flags;
        self
    }

    #[inline]
    pub fn modify_flags<F>(mut self, func: F) -> Self
    where
        F: FnOnce(SwapChainFlags) -> SwapChainFlags,
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
    Format: Format::R8G8B8A8Unorm as u32,
    SampleDesc: DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0,
    },
    BufferUsage: UsageFlags::BACK_BUFFER.0 | UsageFlags::RENDER_TARGET_OUTPUT.0,
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
        impl<'a> $builder<'a> {
            #[inline]
            /// Default is 0x0 (i.e. auto-detect)
            pub fn size(mut self, width: u32, height: u32) -> Self {
                self.desc.Width = width;
                self.desc.Height = height;
                self
            }

            #[inline]
            /// Default RGBA8 UNORM
            pub fn format(mut self, format: Format) -> Self {
                self.desc.Format = format as u32;
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
            pub fn buffer_usage(mut self, usage: UsageFlags) -> Self {
                self.desc.BufferUsage = usage.0;
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
            pub fn scaling(mut self, scaling: Scaling) -> Self {
                self.desc.Scaling = scaling as u32;
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
            pub fn alpha_mode(mut self, mode: AlphaMode) -> Self {
                self.desc.AlphaMode = mode as u32;
                self
            }

            #[inline]
            /// None specified by default
            pub fn flags(mut self, flags: SwapChainFlags) -> Self {
                self.desc.Flags = flags.0;
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
            pub fn scanline_ordering(mut self, order: ModeScanlineOrder) -> Self {
                self.fs_desc.ScanlineOrdering = order as u32;
                self
            }

            #[inline]
            /// Default is UNSPECIFIED
            pub fn fullscreen_scaling(mut self, scaling: ModeScaling) -> Self {
                self.fs_desc.Scaling = scaling as u32;
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

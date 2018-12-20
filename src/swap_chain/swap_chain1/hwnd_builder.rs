use crate::descriptions::{FullscreenDesc, Ratio, SwapChainDesc1};
use crate::device::Device;
use crate::enums::*;
use dcommon::error::Error;
use crate::factory::Factory2;
use crate::output::Output;
use crate::swap_chain::SwapChain1;

use com_wrapper::ComWrapper;
use winapi::shared::windef::HWND;

#[must_use]
/// Builder for a swap chain.
pub struct SwapChainHwndBuilder<'a> {
    factory: &'a Factory2,
    device: &'a Device,
    hwnd: HWND,
    desc: SwapChainDesc1,
    fs_desc: FullscreenDesc,
    restrict_out: Option<&'a Output>,
}

impl<'a> SwapChainHwndBuilder<'a> {
    #[inline]
    pub(crate) fn create(factory: &'a Factory2, device: &'a Device) -> Self {
        SwapChainHwndBuilder {
            factory,
            device,
            hwnd: std::ptr::null_mut(),
            desc: Default::default(),
            fs_desc: Default::default(),
            restrict_out: None,
        }
    }

    #[inline]
    /// Build the swap chain with the provided parameters.
    pub fn build(self) -> Result<SwapChain1, Error> {
        assert!(!self.hwnd.is_null());
        unsafe {
            let factory = self.factory.get_raw();
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory).CreateSwapChainForHwnd(
                self.device.get_raw() as *mut _,
                self.hwnd,
                &self.desc.into(),
                &self.fs_desc.into(),
                self.restrict_out
                    .map(|o| o.get_raw())
                    .unwrap_or(std::ptr::null_mut()),
                &mut ptr,
            );

            Error::map_if(hr, || SwapChain1::from_raw(ptr))
        }
    }

    #[inline]
    /// Required
    pub fn with_hwnd(mut self, hwnd: HWND) -> Self {
        self.hwnd = hwnd;
        self
    }

    #[inline]
    /// Optional
    pub fn with_restrict_output(mut self, out: &'a Output) -> Self {
        self.restrict_out = Some(out);
        self
    }

    #[inline]
    /// Default is 0x0 (i.e. auto-detect)
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.desc.width = width;
        self.desc.height = height;
        self
    }

    #[inline]
    /// Default RGBA8 UNORM
    pub fn with_format(mut self, format: Format) -> Self {
        self.desc.format = format.into();
        self
    }

    #[inline]
    /// Enable MSAA. Default is 1, 0
    pub fn with_samples(mut self, count: u32, quality: u32) -> Self {
        self.desc.sample_desc.count = count;
        self.desc.sample_desc.quality = quality;
        self
    }

    #[inline]
    /// Default is BACK_BUFFER | RENDER_TARGET_OUTPUT
    pub fn with_buffer_usage(mut self, usage: UsageFlags) -> Self {
        self.desc.buffer_usage = usage.into();
        self
    }

    #[inline]
    /// Default is 2
    pub fn with_buffer_count(mut self, count: u32) -> Self {
        self.desc.buffer_count = count;
        self
    }

    #[inline]
    /// Default is Stretch
    pub fn with_scaling(mut self, scaling: Scaling) -> Self {
        self.desc.scaling = scaling.into();
        self
    }

    #[inline]
    /// Default is Discard
    pub fn with_swap_effect(mut self, effect: SwapEffect) -> Self {
        self.desc.swap_effect = effect.into();
        self
    }

    #[inline]
    /// Default is Unspecified
    pub fn with_with_alpha_mode(mut self, mode: AlphaMode) -> Self {
        self.desc.alpha_mode = mode.into();
        self
    }

    #[inline]
    /// None specified by default
    pub fn with_flags(mut self, flags: SwapChainFlags) -> Self {
        self.desc.flags = flags;
        self
    }

    #[inline]
    /// Default is 60/1
    pub fn with_refresh_rate(mut self, hz: Ratio) -> Self {
        self.fs_desc.refresh_rate = hz;
        self
    }

    #[inline]
    /// Default is Unspecified
    pub fn with_scanline_ordering(mut self, order: ModeScanlineOrder) -> Self {
        self.fs_desc.scanline_ordering = order.into();
        self
    }

    #[inline]
    /// Default is Unspecified
    pub fn with_fullscreen_scaling(mut self, scaling: ModeScaling) -> Self {
        self.fs_desc.scaling = scaling.into();
        self
    }

    #[inline]
    /// Default is true
    pub fn with_windowed(mut self, windowed: bool) -> Self {
        self.fs_desc.windowed = windowed.into();
        self
    }
}

use crate::error::Error;
use crate::enums::Format;
use crate::enums::SwapChainFlags;

use checked_enum::UncheckedEnum;
use winapi::shared::dxgi::IDXGISwapChain;

#[must_use]
pub struct ResizeBuffers<'a> {
    pub(super) swap_chain: &'a IDXGISwapChain,
    pub(super) count: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) format: UncheckedEnum<Format>,
    pub(super) flags: SwapChainFlags,
}

impl<'a> ResizeBuffers<'a> {
    #[inline]
    pub fn finish(self) -> Result<(), Error> {
        unsafe {
            let hr = self.swap_chain.ResizeBuffers(
                self.count,
                self.width,
                self.height,
                self.format.value,
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
        self.format = format.into();
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

use descriptions::{DBool, Mode, SampleDesc};
use enums::Format;
use enums::{AlphaMode, Scaling, SwapChainFlags, SwapEffect, UsageFlags};

use checked_enum::UncheckedEnum;
use winapi::shared::dxgi::DXGI_SWAP_CHAIN_DESC;
use winapi::shared::dxgi1_2::DXGI_SWAP_CHAIN_DESC1;
use winapi::shared::windef::HWND;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SwapChainDesc {
    pub buffer_desc: Mode,
    pub sample_desc: SampleDesc,
    pub buffer_usage: UsageFlags,
    pub buffer_count: u32,
    pub output_window: HWND,
    pub windowed: DBool,
    pub swap_effect: UncheckedEnum<SwapEffect>,
    pub flags: SwapChainFlags,
}

#[cfg(test)]
member_compat_test! {
    swap_chain_desc_compat:
    SwapChainDesc <=> DXGI_SWAP_CHAIN_DESC {
        buffer_desc <=> BufferDesc,
        sample_desc <=> SampleDesc,
        buffer_usage <=> BufferUsage,
        buffer_count <=> BufferCount,
        output_window <=> OutputWindow,
        windowed <=> Windowed,
        swap_effect <=> SwapEffect,
        flags <=> Flags,
    }
}

impl From<DXGI_SWAP_CHAIN_DESC> for SwapChainDesc {
    fn from(desc: DXGI_SWAP_CHAIN_DESC) -> SwapChainDesc {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<SwapChainDesc> for DXGI_SWAP_CHAIN_DESC {
    fn from(desc: SwapChainDesc) -> DXGI_SWAP_CHAIN_DESC {
        unsafe { std::mem::transmute(desc) }
    }
}

impl Default for SwapChainDesc {
    fn default() -> Self {
        SwapChainDesc {
            buffer_desc: Default::default(),
            sample_desc: Default::default(),
            buffer_usage: UsageFlags::BACK_BUFFER | UsageFlags::RENDER_TARGET_OUTPUT,
            buffer_count: 2,
            output_window: std::ptr::null_mut(),
            windowed: DBool::TRUE,
            swap_effect: SwapEffect::Discard.into(),
            flags: SwapChainFlags::NONE,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SwapChainDesc1 {
    pub width: u32,
    pub height: u32,
    pub format: UncheckedEnum<Format>,
    pub stereo: DBool,
    pub sample_desc: SampleDesc,
    pub buffer_usage: UsageFlags,
    pub buffer_count: u32,
    pub scaling: UncheckedEnum<Scaling>,
    pub swap_effect: UncheckedEnum<SwapEffect>,
    pub alpha_mode: UncheckedEnum<AlphaMode>,
    pub flags: SwapChainFlags,
}

#[cfg(test)]
member_compat_test! {
    swap_chain_desc1_compat:
    SwapChainDesc1 <=> DXGI_SWAP_CHAIN_DESC1 {
        width <=> Width,
        height <=> Height,
        format <=> Format,
        stereo <=> Stereo,
        sample_desc <=> SampleDesc,
        buffer_usage <=> BufferUsage,
        buffer_count <=> BufferCount,
        scaling <=> Scaling,
        swap_effect <=> SwapEffect,
        alpha_mode <=> AlphaMode,
        flags <=> Flags,
    }
}

impl From<DXGI_SWAP_CHAIN_DESC1> for SwapChainDesc1 {
    fn from(desc: DXGI_SWAP_CHAIN_DESC1) -> SwapChainDesc1 {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<SwapChainDesc1> for DXGI_SWAP_CHAIN_DESC1 {
    fn from(desc: SwapChainDesc1) -> DXGI_SWAP_CHAIN_DESC1 {
        unsafe { std::mem::transmute(desc) }
    }
}

impl Default for SwapChainDesc1 {
    fn default() -> Self {
        SwapChainDesc1 {
            width: 0,
            height: 0,
            format: Format::R8G8B8A8Unorm.into(),
            stereo: DBool::FALSE,
            sample_desc: Default::default(),
            buffer_usage: UsageFlags::BACK_BUFFER | UsageFlags::RENDER_TARGET_OUTPUT,
            buffer_count: 2,
            scaling: Scaling::Stretch.into(),
            swap_effect: SwapEffect::Discard.into(),
            alpha_mode: AlphaMode::Unspecified.into(),
            flags: SwapChainFlags::NONE,
        }
    }
}

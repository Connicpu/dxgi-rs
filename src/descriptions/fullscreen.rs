use crate::descriptions::DBool;
use crate::descriptions::Ratio;
use crate::enums::ModeScaling;
use crate::enums::ModeScanlineOrder;

use checked_enum::UncheckedEnum;
use winapi::shared::dxgi1_2::DXGI_SWAP_CHAIN_FULLSCREEN_DESC;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FullscreenDesc {
    pub refresh_rate: Ratio,
    pub scanline_ordering: UncheckedEnum<ModeScanlineOrder>,
    pub scaling: UncheckedEnum<ModeScaling>,
    pub windowed: DBool,
}

#[cfg(test)]
member_compat_test! {
    fullscreen_desc_compat:
    FullscreenDesc <=> DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
        refresh_rate <=> RefreshRate,
        scanline_ordering <=> ScanlineOrdering,
        scaling <=> Scaling,
        windowed <=> Windowed,
    }
}

impl From<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> for FullscreenDesc {
    #[inline]
    fn from(desc: DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> FullscreenDesc {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<FullscreenDesc> for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    #[inline]
    fn from(desc: FullscreenDesc) -> DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
        unsafe { std::mem::transmute(desc) }
    }
}

impl Default for FullscreenDesc {
    fn default() -> Self {
        FullscreenDesc {
            refresh_rate: 60.into(),
            windowed: DBool::TRUE,
            scaling: ModeScaling::Unspecified.into(),
            scanline_ordering: ModeScanlineOrder::Unspecified.into(),
        }
    }
}

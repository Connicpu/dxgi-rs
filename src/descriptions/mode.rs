use crate::descriptions::Ratio;
use crate::enums::{Format, ModeScaling, ModeScanlineOrder};

use checked_enum::UncheckedEnum;
use winapi::shared::dxgitype::DXGI_MODE_DESC;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: Ratio,
    pub format: UncheckedEnum<Format>,
    pub scanline_ordering: UncheckedEnum<ModeScanlineOrder>,
    pub scaling: UncheckedEnum<ModeScaling>,
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            width: 0,
            height: 0,
            refresh_rate: 60.into(),
            format: Format::R8G8B8A8Unorm.into(),
            scanline_ordering: ModeScanlineOrder::Unspecified.into(),
            scaling: ModeScaling::Unspecified.into(),
        }
    }
}

#[cfg(test)]
member_compat_test! {
    mode_compat:
    Mode <=> DXGI_MODE_DESC {
        width <=> Width,
        height <=> Height,
        refresh_rate <=> RefreshRate,
        format <=> Format,
        scanline_ordering <=> ScanlineOrdering,
        scaling <=> Scaling,
    }
}

impl From<DXGI_MODE_DESC> for Mode {
    fn from(desc: DXGI_MODE_DESC) -> Mode {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<Mode> for DXGI_MODE_DESC {
    fn from(desc: Mode) -> DXGI_MODE_DESC {
        unsafe { std::mem::transmute(desc) }
    }
}

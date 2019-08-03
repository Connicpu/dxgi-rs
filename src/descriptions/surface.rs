use crate::descriptions::SampleDesc;
use crate::enums::Format;

use checked_enum::UncheckedEnum;
use winapi::shared::dxgi::DXGI_SURFACE_DESC;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceDesc {
    pub width: u32,
    pub height: u32,
    pub format: UncheckedEnum<Format>,
    pub sample_desc: SampleDesc,
}

impl Default for SurfaceDesc {
    fn default() -> Self {
        SurfaceDesc {
            width: 0,
            height: 0,
            format: Format::Unknown.into(),
            sample_desc: Default::default(),
        }
    }
}

#[cfg(test)]
member_compat_test! {
    surface_desc_compat:
    SurfaceDesc <=> DXGI_SURFACE_DESC {
        width <=> Width,
        height <=> Height,
        format <=> Format,
        sample_desc <=> SampleDesc,
    }
}

impl From<DXGI_SURFACE_DESC> for SurfaceDesc {
    fn from(desc: DXGI_SURFACE_DESC) -> SurfaceDesc {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<SurfaceDesc> for DXGI_SURFACE_DESC {
    fn from(desc: SurfaceDesc) -> DXGI_SURFACE_DESC {
        unsafe { std::mem::transmute(desc) }
    }
}

use crate::descriptions::DBool;
use crate::enums::ModeRotation;
use crate::helpers::wstrlens;

use checked_enum::UncheckedEnum;
use math2d::Recti;
use winapi::shared::dxgi::DXGI_OUTPUT_DESC;
use winapi::shared::windef::HMONITOR;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OutputDesc {
    pub device_name: [u16; 32],
    pub desktop_coordinates: Recti,
    pub attached_to_desktop: DBool,
    pub rotation: UncheckedEnum<ModeRotation>,
    pub monitor: HMONITOR,
}

impl OutputDesc {
    pub fn device_name(&self) -> String {
        let len = wstrlens(&self.device_name);
        String::from_utf16_lossy(&self.device_name[..len])
    }
}

#[cfg(test)]
member_compat_test! {
    output_desc_compat:
    OutputDesc <=> DXGI_OUTPUT_DESC {
        device_name <=> DeviceName,
        desktop_coordinates <=> DesktopCoordinates,
        attached_to_desktop <=> AttachedToDesktop,
        rotation <=> Rotation,
        monitor <=> Monitor,
    }
}

impl From<DXGI_OUTPUT_DESC> for OutputDesc {
    fn from(desc: DXGI_OUTPUT_DESC) -> OutputDesc {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<OutputDesc> for DXGI_OUTPUT_DESC {
    fn from(desc: OutputDesc) -> DXGI_OUTPUT_DESC {
        unsafe { std::mem::transmute(desc) }
    }
}

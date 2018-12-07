use descriptions::DBool;
use descriptions::Rgb;

use winapi::shared::dxgitype::{DXGI_GAMMA_CONTROL, DXGI_GAMMA_CONTROL_CAPABILITIES};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GammaControl {
    pub scale: Rgb,
    pub offset: Rgb,
    pub gamma_curve: [Rgb; 1025],
}

#[cfg(test)]
member_compat_test! {
    gamma_control_compat:
    GammaControl <=> DXGI_GAMMA_CONTROL {
        scale <=> Scale,
        offset <=> Offset,
        gamma_curve <=> GammaCurve,
    }
}

impl From<DXGI_GAMMA_CONTROL> for GammaControl {
    fn from(desc: DXGI_GAMMA_CONTROL) -> GammaControl {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<GammaControl> for DXGI_GAMMA_CONTROL {
    fn from(desc: GammaControl) -> DXGI_GAMMA_CONTROL {
        unsafe { std::mem::transmute(desc) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GammaControlCaps {
    pub scale_and_offset_supported: DBool,
    pub max_converted_value: f32,
    pub min_converted_value: f32,
    pub num_gamma_control_points: u32,
    pub control_point_positions: [f32; 1025],
}

#[cfg(test)]
member_compat_test! {
    gamma_control_caps_compat:
    GammaControlCaps <=> DXGI_GAMMA_CONTROL_CAPABILITIES {
        scale_and_offset_supported <=> ScaleAndOffsetSupported,
        max_converted_value <=> MaxConvertedValue,
        min_converted_value <=> MinConvertedValue,
        num_gamma_control_points <=> NumGammaControlPoints,
        control_point_positions <=> ControlPointPositions,
    }
}

impl From<DXGI_GAMMA_CONTROL_CAPABILITIES> for GammaControlCaps {
    fn from(desc: DXGI_GAMMA_CONTROL_CAPABILITIES) -> GammaControlCaps {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<GammaControlCaps> for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn from(desc: GammaControlCaps) -> DXGI_GAMMA_CONTROL_CAPABILITIES {
        unsafe { std::mem::transmute(desc) }
    }
}

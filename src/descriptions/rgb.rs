use winapi::shared::dxgitype::DXGI_RGB;
use winapi::shared::dxgitype::DXGI_RGBA;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[cfg(test)]
member_compat_test! {
    rgb_compat:
    Rgb <=> DXGI_RGB {
        r <=> Red,
        g <=> Green,
        b <=> Blue,
    }
}

impl From<DXGI_RGB> for Rgb {
    fn from(desc: DXGI_RGB) -> Rgb {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<Rgb> for DXGI_RGB {
    fn from(desc: Rgb) -> DXGI_RGB {
        unsafe { std::mem::transmute(desc) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[cfg(test)]
member_compat_test! {
    rgba_compat:
    Rgba <=> DXGI_RGBA {
        r <=> r,
        g <=> g,
        b <=> b,
        a <=> a,
    }
}

impl From<DXGI_RGBA> for Rgba {
    fn from(desc: DXGI_RGBA) -> Rgba {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<Rgba> for DXGI_RGBA {
    fn from(desc: Rgba) -> DXGI_RGBA {
        unsafe { std::mem::transmute(desc) }
    }
}

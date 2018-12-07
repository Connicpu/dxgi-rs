use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SampleDesc {
    pub count: u32,
    pub quality: u32,
}

impl Default for SampleDesc {
    fn default() -> Self {
        SampleDesc {
            count: 1,
            quality: 0,
        }
    }
}

#[cfg(test)]
member_compat_test! {
    sample_desc_compat:
    SampleDesc <=> DXGI_SAMPLE_DESC {
        count <=> Count,
        quality <=> Quality,
    }
}

impl From<DXGI_SAMPLE_DESC> for SampleDesc {
    fn from(desc: DXGI_SAMPLE_DESC) -> SampleDesc {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<SampleDesc> for DXGI_SAMPLE_DESC {
    fn from(desc: SampleDesc) -> DXGI_SAMPLE_DESC {
        unsafe { std::mem::transmute(desc) }
    }
}

use winapi::shared::dxgi1_5::DXGI_FEATURE_PRESENT_ALLOW_TEARING;
use winapi::shared::minwindef::BOOL;
use winapi::shared::winerror::SUCCEEDED;

#[derive(Copy, Clone)]
pub struct AllowTearing;

unsafe impl super::Feature for AllowTearing {
    const FLAG: u32 = DXGI_FEATURE_PRESENT_ALLOW_TEARING;

    type Structure = BOOL;
    type Result = bool;

    fn get_result(hr: i32, allow_tearing: &BOOL) -> bool {
        // Based on https://docs.microsoft.com/en-us/windows/desktop/direct3ddxgi/variable-refresh-rate-displays
        SUCCEEDED(hr) && *allow_tearing != 0
    }
}

use crate::descriptions::AdapterDesc2;
use crate::helpers::deref_com_wrapper;

use winapi::shared::winerror::SUCCEEDED;
use winapi::shared::dxgi1_2::IDXGIAdapter2;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync)]
pub struct Adapter2 {
    ptr: ComPtr<IDXGIAdapter2>,
}

impl Adapter2 {
    pub fn desc(&self) -> AdapterDesc2 {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.ptr.GetDesc2(&mut desc);
            assert!(
                SUCCEEDED(hr),
                "hr that shouldn't fail, failed: {}",
                crate::error::Error(hr).get_message()
            );
            desc.into()
        }
    }
}

impl std::ops::Deref for Adapter2 {
    type Target = super::Adapter1;
    fn deref(&self) -> &Self::Target {
        unsafe { deref_com_wrapper(self) }
    }
}

impl super::AdapterType for Adapter2 {}

use crate::descriptions::AdapterDesc3;
use crate::helpers::deref_com_wrapper;

use dcommon::error::Error;
use winapi::shared::dxgi1_6::IDXGIAdapter4;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync)]
pub struct Adapter4 {
    ptr: ComPtr<IDXGIAdapter4>,
}

impl Adapter4 {
    pub fn desc(&self) -> AdapterDesc3 {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.ptr.GetDesc3(&mut desc);
            assert!(
                SUCCEEDED(hr),
                "hr that shouldn't fail, failed: {:?}",
                Error(hr)
            );
            desc.into()
        }
    }
}

impl std::ops::Deref for Adapter4 {
    type Target = super::Adapter3;
    fn deref(&self) -> &Self::Target {
        unsafe { deref_com_wrapper(self) }
    }
}

impl super::AdapterType for Adapter4 {}

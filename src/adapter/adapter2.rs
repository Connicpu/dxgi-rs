use crate::adapter::AdapterType;
use crate::adapter::{IAdapter, IAdapter1};
use crate::descriptions::AdapterDesc2;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::{IDXGIAdapter, IDXGIAdapter1};
use winapi::shared::dxgi1_2::IDXGIAdapter2;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync)]
pub struct Adapter2 {
    ptr: ComPtr<IDXGIAdapter2>,
}

pub unsafe trait IAdapter2: IAdapter1 {
    fn desc2(&self) -> AdapterDesc2 {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.raw_adp2().GetDesc2(&mut desc);
            assert!(
                SUCCEEDED(hr),
                "hr that shouldn't fail, failed: {:?}",
                Error(hr)
            );
            desc.into()
        }
    }

    unsafe fn raw_adp2(&self) -> &IDXGIAdapter2;
}

unsafe impl IAdapter for Adapter2 {
    unsafe fn raw_adp(&self) -> &IDXGIAdapter {
        &self.ptr
    }
}

unsafe impl IAdapter1 for Adapter2 {
    unsafe fn raw_adp1(&self) -> &IDXGIAdapter1 {
        &self.ptr
    }
}

unsafe impl IAdapter2 for Adapter2 {
    unsafe fn raw_adp2(&self) -> &IDXGIAdapter2 {
        &self.ptr
    }
}

unsafe impl AdapterType for Adapter2 {}

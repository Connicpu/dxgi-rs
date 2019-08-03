use crate::adapter::AdapterType;
use crate::adapter::{IAdapter, IAdapter1, IAdapter2, IAdapter3};
use crate::descriptions::AdapterDesc3;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::{IDXGIAdapter, IDXGIAdapter1};
use winapi::shared::dxgi1_2::IDXGIAdapter2;
use winapi::shared::dxgi1_4::IDXGIAdapter3;
use winapi::shared::dxgi1_6::IDXGIAdapter4;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync)]
pub struct Adapter4 {
    ptr: ComPtr<IDXGIAdapter4>,
}

pub unsafe trait IAdapter4: IAdapter3 {
    fn desc3(&self) -> AdapterDesc3 {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.raw_adp4().GetDesc3(&mut desc);
            assert!(
                SUCCEEDED(hr),
                "hr that shouldn't fail, failed: {:?}",
                Error(hr)
            );
            desc.into()
        }
    }

    unsafe fn raw_adp4(&self) -> &IDXGIAdapter4;
}

unsafe impl IAdapter for Adapter4 {
    unsafe fn raw_adp(&self) -> &IDXGIAdapter {
        &self.ptr
    }
}

unsafe impl IAdapter1 for Adapter4 {
    unsafe fn raw_adp1(&self) -> &IDXGIAdapter1 {
        &self.ptr
    }
}

unsafe impl IAdapter2 for Adapter4 {
    unsafe fn raw_adp2(&self) -> &IDXGIAdapter2 {
        &self.ptr
    }
}

unsafe impl IAdapter3 for Adapter4 {
    unsafe fn raw_adp3(&self) -> &IDXGIAdapter3 {
        &self.ptr
    }
}

unsafe impl IAdapter4 for Adapter4 {
    unsafe fn raw_adp4(&self) -> &IDXGIAdapter4 {
        &self.ptr
    }
}

unsafe impl AdapterType for Adapter4 {}

use winapi::shared::dxgi::{IDXGIDevice, IDXGIDevice1};
use wio::com::ComPtr;

use crate::device::IDevice;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device1 {
    ptr: ComPtr<IDXGIDevice1>,
}

pub unsafe trait IDevice1 {
    unsafe fn raw_dev1(&self) -> &IDXGIDevice1;
}

unsafe impl IDevice for Device1 {
    unsafe fn raw_dev(&self) -> &IDXGIDevice {
        &self.ptr
    }
}

unsafe impl IDevice1 for Device1 {
    unsafe fn raw_dev1(&self) -> &IDXGIDevice1 {
        &self.ptr
    }
}

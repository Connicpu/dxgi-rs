use winapi::shared::dxgi::{IDXGIDevice, IDXGIDevice1};
use winapi::shared::dxgi1_2::IDXGIDevice2;
use wio::com::ComPtr;

use crate::device::{IDevice, IDevice1};

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device2 {
    ptr: ComPtr<IDXGIDevice2>,
}

pub unsafe trait IDevice2 {
    unsafe fn raw_dev2(&self) -> &IDXGIDevice2;
}

unsafe impl IDevice for Device2 {
    unsafe fn raw_dev(&self) -> &IDXGIDevice {
        &self.ptr
    }
}

unsafe impl IDevice1 for Device2 {
    unsafe fn raw_dev1(&self) -> &IDXGIDevice1 {
        &self.ptr
    }
}

unsafe impl IDevice2 for Device2 {
    unsafe fn raw_dev2(&self) -> &IDXGIDevice2 {
        &self.ptr
    }
}

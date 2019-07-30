use winapi::shared::dxgi::{IDXGIDevice, IDXGIDevice1};
use winapi::shared::dxgi1_2::IDXGIDevice2;
use winapi::shared::dxgi1_3::IDXGIDevice3;
use wio::com::ComPtr;

use crate::device::{IDevice, IDevice1, IDevice2};

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device3 {
    ptr: ComPtr<IDXGIDevice3>,
}

pub unsafe trait IDevice3 {
    unsafe fn raw_dev3(&self) -> &IDXGIDevice3;
}

unsafe impl IDevice for Device3 {
    unsafe fn raw_dev(&self) -> &IDXGIDevice {
        &self.ptr
    }
}

unsafe impl IDevice1 for Device3 {
    unsafe fn raw_dev1(&self) -> &IDXGIDevice1 {
        &self.ptr
    }
}

unsafe impl IDevice2 for Device3 {
    unsafe fn raw_dev2(&self) -> &IDXGIDevice2 {
        &self.ptr
    }
}

unsafe impl IDevice3 for Device3 {
    unsafe fn raw_dev3(&self) -> &IDXGIDevice3 {
        &self.ptr
    }
}

use crate::device::{IDevice, IDevice1, IDevice2, IDevice3};

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIDevice, IDXGIDevice1};
use winapi::shared::dxgi1_2::IDXGIDevice2;
use winapi::shared::dxgi1_3::IDXGIDevice3;
use winapi::shared::dxgi1_5::IDXGIDevice4;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device4 {
    ptr: ComPtr<IDXGIDevice4>,
}

pub unsafe trait IDevice4 {
    unsafe fn raw_dev4(&self) -> &IDXGIDevice4;
}

unsafe impl IDevice for Device4 {
    unsafe fn raw_dev(&self) -> &IDXGIDevice {
        &self.ptr
    }
}

unsafe impl IDevice1 for Device4 {
    unsafe fn raw_dev1(&self) -> &IDXGIDevice1 {
        &self.ptr
    }
}

unsafe impl IDevice2 for Device4 {
    unsafe fn raw_dev2(&self) -> &IDXGIDevice2 {
        &self.ptr
    }
}

unsafe impl IDevice3 for Device4 {
    unsafe fn raw_dev3(&self) -> &IDXGIDevice3 {
        &self.ptr
    }
}

unsafe impl IDevice4 for Device4 {
    unsafe fn raw_dev4(&self) -> &IDXGIDevice4 {
        &self.ptr
    }
}

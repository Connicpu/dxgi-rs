use crate::enums::FactoryCreationFlags;
use crate::factory::{FactoryType, IFactory, IFactory1, IFactory2};

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIFactory, IDXGIFactory1};
use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::dxgi1_3::IDXGIFactory3;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory3 {
    ptr: ComPtr<IDXGIFactory3>,
}

pub unsafe trait IFactory3: IFactory2 {
    fn creation_flags(&self) -> FactoryCreationFlags {
        unsafe { FactoryCreationFlags(self.raw_f3().GetCreationFlags()) }
    }

    unsafe fn raw_f3(&self) -> &IDXGIFactory3;
}

unsafe impl IFactory for Factory3 {
    unsafe fn raw_f(&self) -> &IDXGIFactory {
        &self.ptr
    }
}

unsafe impl IFactory1 for Factory3 {
    unsafe fn raw_f1(&self) -> &IDXGIFactory1 {
        &self.ptr
    }
}

unsafe impl IFactory2 for Factory3 {
    unsafe fn raw_f2(&self) -> &IDXGIFactory2 {
        &self.ptr
    }
}

unsafe impl IFactory3 for Factory3 {
    unsafe fn raw_f3(&self) -> &IDXGIFactory3 {
        &self.ptr
    }
}

unsafe impl FactoryType for Factory3 {}

use crate::adapter::AdapterType;
use crate::factory::{FactoryType, IFactory, IFactory1, IFactory2, IFactory3};

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::{IDXGIFactory, IDXGIFactory1};
use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::dxgi1_3::IDXGIFactory3;
use winapi::shared::dxgi1_4::IDXGIFactory4;
use winapi::um::winnt::LUID;
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory4 {
    ptr: ComPtr<IDXGIFactory4>,
}

pub unsafe trait IFactory4: IFactory3 {
    fn adapter_by_luid<A: AdapterType>(&self, luid: i64) -> Result<A, Error>
    where
        Self: Sized,
    {
        imp_adapter_by_luid(self, luid)
    }

    fn warp_adapter<A: AdapterType>(&self) -> Result<A, Error>
    where
        Self: Sized,
    {
        imp_warp_adapter(self)
    }

    unsafe fn raw_f4(&self) -> &IDXGIFactory4;
}

impl dyn IFactory4 + '_ {
    pub fn adapter_by_luid_dyn<A: AdapterType>(&self, luid: i64) -> Result<A, Error> {
        imp_adapter_by_luid(self, luid)
    }

    pub fn warp_adapter_dyn<A: AdapterType>(&self) -> Result<A, Error> {
        imp_warp_adapter(self)
    }
}

fn imp_adapter_by_luid<A: AdapterType>(f: &dyn IFactory4, luid: i64) -> Result<A, Error> {
    let luid = LUID {
        HighPart: (luid >> 32) as i32,
        LowPart: luid as u32,
    };

    unsafe {
        let mut ptr = std::ptr::null_mut();
        let hr = f
            .raw_f4()
            .EnumAdapterByLuid(luid, &A::Interface::uuidof(), &mut ptr);
        Error::map_if(hr, || A::from_raw(ptr as _))
    }
}

fn imp_warp_adapter<A: AdapterType>(f: &dyn IFactory4) -> Result<A, Error> {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        let hr = f
            .raw_f4()
            .EnumWarpAdapter(&A::Interface::uuidof(), &mut ptr);
        Error::map_if(hr, || A::from_raw(ptr as _))
    }
}

unsafe impl IFactory for Factory4 {
    unsafe fn raw_f(&self) -> &IDXGIFactory {
        &self.ptr
    }
}

unsafe impl IFactory1 for Factory4 {
    unsafe fn raw_f1(&self) -> &IDXGIFactory1 {
        &self.ptr
    }
}

unsafe impl IFactory2 for Factory4 {
    unsafe fn raw_f2(&self) -> &IDXGIFactory2 {
        &self.ptr
    }
}

unsafe impl IFactory3 for Factory4 {
    unsafe fn raw_f3(&self) -> &IDXGIFactory3 {
        &self.ptr
    }
}

unsafe impl IFactory4 for Factory4 {
    unsafe fn raw_f4(&self) -> &IDXGIFactory4 {
        &self.ptr
    }
}

unsafe impl FactoryType for Factory4 {}

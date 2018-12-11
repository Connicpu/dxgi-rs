use crate::adapter::AdapterType;
use crate::error::Error;

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

impl Factory4 {
    pub fn adapter_by_luid<A: AdapterType>(&self, luid: i64) -> Result<A, Error> {
        let luid = LUID {
            HighPart: (luid >> 32) as i32,
            LowPart: luid as u32,
        };

        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self
                .ptr
                .EnumAdapterByLuid(luid, &A::Interface::uuidof(), &mut ptr);
            Error::map_if(hr, || A::from_raw(ptr as _))
        }
    }

    pub fn warp_adapter<A: AdapterType>(&self) -> Result<A, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.EnumWarpAdapter(&A::Interface::uuidof(), &mut ptr);
            Error::map_if(hr, || A::from_raw(ptr as _))
        }
    }
}

impl super::FactoryType for Factory4 {}

impl std::ops::Deref for Factory4 {
    type Target = super::Factory3;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Factory4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

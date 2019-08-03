use crate::adapter::AdapterType;
use crate::adapter::{IAdapter, IAdapter1, IAdapter2};

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::{IDXGIAdapter, IDXGIAdapter1};
use winapi::shared::dxgi1_2::IDXGIAdapter2;
use winapi::shared::dxgi1_4::IDXGIAdapter3;
use wio::com::ComPtr;

#[doc(inline)]
pub use self::register::*;

mod register;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync)]
pub struct Adapter3 {
    ptr: ComPtr<IDXGIAdapter3>,
}

pub unsafe trait IAdapter3: IAdapter2 {
    fn register_hcpt_event(&self, event: &dyn EventHandle) -> Result<HcptStatusCookie, Error> {
        unsafe {
            let mut c = 0;
            let hr = self
                .raw_adp3()
                .RegisterHardwareContentProtectionTeardownStatusEvent(event.get_handle(), &mut c);
            Error::map_if(hr, || HcptStatusCookie(c))
        }
    }

    fn register_vmbc_event(&self, event: &dyn EventHandle) -> Result<VmbcStatusCookie, Error> {
        unsafe {
            let mut c = 0;
            let hr = self
                .raw_adp3()
                .RegisterVideoMemoryBudgetChangeNotificationEvent(event.get_handle(), &mut c);
            Error::map_if(hr, || VmbcStatusCookie(c))
        }
    }

    unsafe fn raw_adp3(&self) -> &IDXGIAdapter3;
}

unsafe impl IAdapter for Adapter3 {
    unsafe fn raw_adp(&self) -> &IDXGIAdapter {
        &self.ptr
    }
}

unsafe impl IAdapter1 for Adapter3 {
    unsafe fn raw_adp1(&self) -> &IDXGIAdapter1 {
        &self.ptr
    }
}

unsafe impl IAdapter2 for Adapter3 {
    unsafe fn raw_adp2(&self) -> &IDXGIAdapter2 {
        &self.ptr
    }
}

unsafe impl IAdapter3 for Adapter3 {
    unsafe fn raw_adp3(&self) -> &IDXGIAdapter3 {
        &self.ptr
    }
}

unsafe impl AdapterType for Adapter3 {}

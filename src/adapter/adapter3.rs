use dcommon::error::Error;
use crate::helpers::deref_com_wrapper;

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

impl Adapter3 {
    pub fn register_hcpt_event(&self, event: &impl EventHandle) -> Result<HcptStatusCookie, Error> {
        unsafe {
            let mut c = 0;
            let hr = self
                .ptr
                .RegisterHardwareContentProtectionTeardownStatusEvent(event.get_handle(), &mut c);
            Error::map_if(hr, || HcptStatusCookie(c))
        }
    }

    pub fn register_vmbc_event(&self, event: &impl EventHandle) -> Result<VmbcStatusCookie, Error> {
        unsafe {
            let mut c = 0;
            let hr = self
                .ptr
                .RegisterVideoMemoryBudgetChangeNotificationEvent(event.get_handle(), &mut c);
            Error::map_if(hr, || VmbcStatusCookie(c))
        }
    }
}

impl std::ops::Deref for Adapter3 {
    type Target = super::Adapter2;
    fn deref(&self) -> &Self::Target {
        unsafe { deref_com_wrapper(self) }
    }
}

impl super::AdapterType for Adapter3 {}

use adapter::Adapter;
use error::Error;

use std::ptr;

use winapi::Interface;
use winapi::shared::dxgi::{IDXGIDevice, IDXGIAdapter1};
use wio::com::ComPtr;

pub struct Device {
    ptr: ComPtr<IDXGIDevice>,
}

impl Device {
    #[inline]
    pub unsafe fn from_raw(ptr: *mut IDXGIDevice) -> Device {
        Device {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut IDXGIDevice {
        self.ptr.as_raw()
    }

    #[inline]
    pub fn get_adapter(&self) -> Result<Adapter, Error> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetAdapter(&mut ptr);
            Error::map(hr, ())?;
            let mut ptr1 = ptr::null_mut();
            let hr = (*ptr).QueryInterface(&IDXGIAdapter1::uuidof(), &mut ptr1);
            Error::map_if(hr, || Adapter::from_raw(ptr1 as *mut _))
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

use crate::device_subobject::IDeviceSubObject;
use crate::enums::ResourcePriority;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIDeviceSubObject, IDXGIResource};
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Resource {
    ptr: ComPtr<IDXGIResource>,
}

pub unsafe trait IResource: IDeviceSubObject {
    /// Get the eviction priority, which determines when a resource can be evicted from memory.
    fn eviction_priority(&self) -> Option<ResourcePriority> {
        unsafe {
            let mut pri = 0;
            let hr = self.raw_res().GetEvictionPriority(&mut pri);
            if SUCCEEDED(hr) {
                Some(ResourcePriority(pri))
            } else {
                None
            }
        }
    }

    unsafe fn raw_res(&self) -> &IDXGIResource;
}

unsafe impl IDeviceSubObject for Resource {
    unsafe fn raw_dso(&self) -> &IDXGIDeviceSubObject {
        &self.ptr
    }
}

unsafe impl IResource for Resource {
    unsafe fn raw_res(&self) -> &IDXGIResource {
        &self.ptr
    }
}

use enums::ResourcePriority;

use winapi::shared::dxgi::IDXGIResource;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Resource {
    ptr: ComPtr<IDXGIResource>,
}

impl Resource {
    /// Get the eviction priority, which determines when a resource can be evicted from memory.
    pub fn eviction_priority(&self) -> Option<ResourcePriority> {
        unsafe {
            let mut pri = 0;
            let hr = self.ptr.GetEvictionPriority(&mut pri);
            if SUCCEEDED(hr) {
                Some(ResourcePriority(pri))
            } else {
                None
            }
        }
    }
}

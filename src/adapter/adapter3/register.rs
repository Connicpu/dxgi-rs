use crate::adapter::adapter3::IAdapter3;

use winapi::shared::ntdef::HANDLE;

pub unsafe trait EventHandle {
    fn get_handle(&self) -> HANDLE;
}

/// Hardware Content Protection Teardown status event cookie
pub struct HcptStatusCookie(pub(super) u32);

impl HcptStatusCookie {
    pub fn unregister(self, adapter: &dyn IAdapter3) {
        unsafe {
            let ptr = &*adapter.raw_adp3();
            ptr.UnregisterHardwareContentProtectionTeardownStatus(self.0);
        }
    }
}

/// Video Memory Budget Changed status event cookie
pub struct VmbcStatusCookie(pub(super) u32);

impl VmbcStatusCookie {
    pub fn unregister(self, adapter: &dyn IAdapter3) {
        unsafe {
            let ptr = &*adapter.raw_adp3();
            ptr.UnregisterVideoMemoryBudgetChangeNotification(self.0);
        }
    }
}

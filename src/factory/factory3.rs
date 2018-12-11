use crate::enums::FactoryCreationFlags;

use winapi::shared::dxgi1_3::IDXGIFactory3;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory3 {
    ptr: ComPtr<IDXGIFactory3>,
}

impl Factory3 {
    pub fn creation_flags(&self) -> FactoryCreationFlags {
        unsafe { FactoryCreationFlags(self.ptr.GetCreationFlags()) }
    }
}

impl super::FactoryType for Factory3 {}

impl std::ops::Deref for Factory3 {
    type Target = super::Factory2;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Factory3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

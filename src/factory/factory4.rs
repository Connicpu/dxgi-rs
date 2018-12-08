use winapi::shared::dxgi1_4::IDXGIFactory4;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory4 {
    ptr: ComPtr<IDXGIFactory4>,
}

impl Factory4 {}

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

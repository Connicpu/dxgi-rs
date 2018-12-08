use winapi::shared::dxgi1_5::IDXGIFactory5;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory5 {
    ptr: ComPtr<IDXGIFactory5>,
}

impl Factory5 {}

impl super::FactoryType for Factory5 {}

impl std::ops::Deref for Factory5 {
    type Target = super::Factory4;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Factory5 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

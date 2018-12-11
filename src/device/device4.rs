use winapi::shared::dxgi1_5::IDXGIDevice4;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device4 {
    ptr: ComPtr<IDXGIDevice4>,
}

impl super::DeviceType for Device4 {}

impl std::ops::Deref for Device4 {
    type Target = super::Device3;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Device4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

use winapi::shared::dxgi1_3::IDXGIDevice3;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device3 {
    ptr: ComPtr<IDXGIDevice3>,
}

impl super::DeviceType for Device3 {}

impl std::ops::Deref for Device3 {
    type Target = super::Device2;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Device3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

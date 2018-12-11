use winapi::shared::dxgi1_2::IDXGIDevice2;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device2 {
    ptr: ComPtr<IDXGIDevice2>,
}

impl super::DeviceType for Device2 {}

impl std::ops::Deref for Device2 {
    type Target = super::Device1;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Device2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

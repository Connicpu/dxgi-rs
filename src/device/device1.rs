use winapi::shared::dxgi::IDXGIDevice1;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device1 {
    ptr: ComPtr<IDXGIDevice1>,
}

impl super::DeviceType for Device1 {}

impl std::ops::Deref for Device1 {
    type Target = super::Device;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Device1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

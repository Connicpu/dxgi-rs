use crate::features::Feature;

use winapi::shared::dxgi1_5::IDXGIFactory5;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory5 {
    ptr: ComPtr<IDXGIFactory5>,
}

impl Factory5 {
    pub fn check_feature_support<F: Feature>(&self) -> F::Result {
        unsafe {
            let mut data: F::Structure = std::mem::zeroed();
            let hr = self.ptr.CheckFeatureSupport(
                F::FLAG,
                (&mut data) as *mut _ as _,
                std::mem::size_of_val(&data) as u32,
            );
            F::get_result(hr, &data)
        }
    }
}

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

use crate::error::Error;

use winapi::shared::dxgi1_2::IDXGIFactory2;
use wio::com::ComPtr;

use self::register::RegisterStatus;

pub use self::register::RegisterStatusToken;
pub use self::register::StatusEventCookie;
pub use self::register::StatusEventReceiver;

mod register;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The Factory2 interface is required to create a newer version swap chain with more
/// features than SwapChain and to monitor stereoscopic 3D capabilities.
///
/// Supported: Windows 8 and Platform Update for Windows 7
pub struct Factory2 {
    ptr: ComPtr<IDXGIFactory2>,
}

impl Factory2 {
    pub fn is_windowed_stereo_enabled(&self) -> bool {
        unsafe { self.ptr.IsWindowedStereoEnabled() != 0 }
    }

    pub fn register_occlusion_status(
        &self,
        receiver: impl StatusEventReceiver,
    ) -> Result<StatusEventCookie, Error> {
        receiver.register(RegisterStatusToken {
            factory: &self.ptr,
            event: RegisterStatus::Occlusion,
        })
    }

    pub fn register_stereo_status(
        &self,
        receiver: impl StatusEventReceiver,
    ) -> Result<StatusEventCookie, Error> {
        receiver.register(RegisterStatusToken {
            factory: &self.ptr,
            event: RegisterStatus::Stereo,
        })
    }

    pub fn unregister_status(&self, cookie: StatusEventCookie) {
        unsafe {
            match cookie.1 {
                RegisterStatus::Stereo => self.ptr.UnregisterStereoStatus(cookie.0),
                RegisterStatus::Occlusion => self.ptr.UnregisterOcclusionStatus(cookie.0),
            }
        }
    }
}

impl super::FactoryType for Factory2 {}

impl std::ops::Deref for Factory2 {
    type Target = super::Factory1;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Factory2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}

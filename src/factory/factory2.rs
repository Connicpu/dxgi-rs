use crate::factory::{FactoryType, IFactory, IFactory1};

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::{IDXGIFactory, IDXGIFactory1};
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

pub unsafe trait IFactory2: IFactory1 {
    fn is_windowed_stereo_enabled(&self) -> bool {
        unsafe { self.raw_f2().IsWindowedStereoEnabled() != 0 }
    }

    fn register_occlusion_status(
        &self,
        receiver: impl StatusEventReceiver,
    ) -> Result<StatusEventCookie, Error>
    where
        Self: Sized,
    {
        receiver.register(RegisterStatusToken {
            factory: unsafe { self.raw_f2() },
            event: RegisterStatus::Occlusion,
        })
    }

    fn register_stereo_status(
        &self,
        receiver: impl StatusEventReceiver,
    ) -> Result<StatusEventCookie, Error>
    where
        Self: Sized,
    {
        receiver.register(RegisterStatusToken {
            factory: unsafe { self.raw_f2() },
            event: RegisterStatus::Stereo,
        })
    }

    fn unregister_status(&self, cookie: StatusEventCookie) {
        unsafe {
            match cookie.1 {
                RegisterStatus::Stereo => self.raw_f2().UnregisterStereoStatus(cookie.0),
                RegisterStatus::Occlusion => self.raw_f2().UnregisterOcclusionStatus(cookie.0),
            }
        }
    }

    unsafe fn raw_f2(&self) -> &IDXGIFactory2;
}

impl dyn IFactory2 + '_ {
    pub fn register_occlusion_status_dyn(
        &self,
        receiver: impl StatusEventReceiver,
    ) -> Result<StatusEventCookie, Error> {
        receiver.register(RegisterStatusToken {
            factory: unsafe { self.raw_f2() },
            event: RegisterStatus::Occlusion,
        })
    }

    pub fn register_stereo_status_dyn(
        &self,
        receiver: impl StatusEventReceiver,
    ) -> Result<StatusEventCookie, Error> {
        receiver.register(RegisterStatusToken {
            factory: unsafe { self.raw_f2() },
            event: RegisterStatus::Stereo,
        })
    }
}

unsafe impl IFactory for Factory2 {
    unsafe fn raw_f(&self) -> &IDXGIFactory {
        &self.ptr
    }
}

unsafe impl IFactory1 for Factory2 {
    unsafe fn raw_f1(&self) -> &IDXGIFactory1 {
        &self.ptr
    }
}

unsafe impl IFactory2 for Factory2 {
    unsafe fn raw_f2(&self) -> &IDXGIFactory2 {
        &self.ptr
    }
}

unsafe impl FactoryType for Factory2 {}

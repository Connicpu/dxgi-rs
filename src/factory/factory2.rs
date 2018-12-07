use error::Error;
use factory::FactoryType;

use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

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

    pub fn register_status(
        &self,
        event: RegisterStatus,
        receiver: impl StatusEventReceiver,
    ) -> Result<StatusEventCookie, Error> {
        receiver.register(RegisterStatusToken {
            factory: &self.ptr,
            event,
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

impl FactoryType for Factory2 {}

pub trait StatusEventReceiver {
    fn register(self, token: RegisterStatusToken) -> Result<StatusEventCookie, Error>;
}

#[derive(Copy, Clone, Debug)]
pub struct StatusEventCookie(u32, RegisterStatus);

pub struct RegisterStatusToken<'a> {
    factory: &'a IDXGIFactory2,
    event: RegisterStatus,
}

#[derive(Copy, Clone, Debug)]
pub enum RegisterStatus {
    Stereo,
    Occlusion,
}

impl<'a> RegisterStatusToken<'a> {
    pub unsafe fn register_window(
        self,
        window: HWND,
        message: u32,
    ) -> Result<StatusEventCookie, Error> {
        let mut cookie = 0;
        let hr = match self.event {
            RegisterStatus::Stereo => {
                self.factory
                    .RegisterStereoStatusWindow(window, message, &mut cookie)
            }
            RegisterStatus::Occlusion => {
                self.factory
                    .RegisterOcclusionStatusWindow(window, message, &mut cookie)
            }
        };
        if SUCCEEDED(hr) {
            Ok(StatusEventCookie(cookie, self.event))
        } else {
            Err(hr.into())
        }
    }

    pub unsafe fn register_event(self, event: HANDLE) -> Result<StatusEventCookie, Error> {
        let mut cookie = 0;
        let hr = match self.event {
            RegisterStatus::Stereo => self.factory.RegisterStereoStatusEvent(event, &mut cookie),
            RegisterStatus::Occlusion => self
                .factory
                .RegisterOcclusionStatusEvent(event, &mut cookie),
        };
        if SUCCEEDED(hr) {
            Ok(StatusEventCookie(cookie, self.event))
        } else {
            Err(hr.into())
        }
    }
}

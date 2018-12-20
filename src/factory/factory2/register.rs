use dcommon::error::Error;

use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::SUCCEEDED;

pub trait StatusEventReceiver {
    fn register(self, token: RegisterStatusToken) -> Result<StatusEventCookie, Error>;
}

#[derive(Copy, Clone, Debug)]
pub struct StatusEventCookie(pub(crate) u32, pub(crate) RegisterStatus);

pub struct RegisterStatusToken<'a> {
    pub(crate) factory: &'a IDXGIFactory2,
    pub(crate) event: RegisterStatus,
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

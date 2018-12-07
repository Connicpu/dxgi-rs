use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGIDeviceSubObject;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct DeviceSubObject {
    ptr: ComPtr<IDXGIDeviceSubObject>,
}

pub trait GraphicsDevice: ComWrapper {}

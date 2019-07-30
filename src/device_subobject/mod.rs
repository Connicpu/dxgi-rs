use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGIDeviceSubObject;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct DeviceSubObject {
    ptr: ComPtr<IDXGIDeviceSubObject>,
}

pub unsafe trait IDeviceSubObject {
    unsafe fn raw_dso(&self) -> &IDXGIDeviceSubObject;
}

unsafe impl IDeviceSubObject for DeviceSubObject {
    unsafe fn raw_dso(&self) -> &IDXGIDeviceSubObject {
        &self.ptr
    }
}

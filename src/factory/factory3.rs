use winapi::shared::dxgi1_3::IDXGIFactory3;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The Factory2 interface is required to create a newer version swap chain with more
/// features than SwapChain and to monitor stereoscopic 3D capabilities.
/// 
/// Supported: Windows 8 and Platform Update for Windows 7
pub struct Factory3 {
    ptr: ComPtr<IDXGIFactory3>,
}

impl Factory3 {
}

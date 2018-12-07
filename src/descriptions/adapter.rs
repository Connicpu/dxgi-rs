use enums::AdapterFlags;
use helpers::wstrlens;

use winapi::shared::dxgi::DXGI_ADAPTER_DESC;
use winapi::shared::dxgi::DXGI_ADAPTER_DESC1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AdapterDesc {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub subsys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: i64,
}

impl AdapterDesc {
    pub fn description(&self) -> String {
        let len = wstrlens(&self.description);
        String::from_utf16_lossy(&self.description[..len])
    }
}

impl std::fmt::Debug for AdapterDesc {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AdapterDesc")
            .field("description", &self.description())
            .field("vendor_id", &self.vendor_id)
            .field("device_id", &self.device_id)
            .field("subsys_id", &self.subsys_id)
            .field("revision", &self.revision)
            .field("dedicated_video_memory", &self.dedicated_video_memory)
            .field("dedicated_system_memory", &self.dedicated_system_memory)
            .field("shared_system_memory", &self.shared_system_memory)
            .field("adapter_luid", &self.adapter_luid)
            .finish()
    }
}

#[cfg(test)]
member_compat_test! {
    adapter_desc_compat:
    AdapterDesc <=> DXGI_ADAPTER_DESC {
        description <=> Description,
        vendor_id <=> VendorId,
        device_id <=> DeviceId,
        subsys_id <=> SubSysId,
        revision <=> Revision,
        dedicated_video_memory <=> DedicatedVideoMemory,
        dedicated_system_memory <=> DedicatedSystemMemory,
        shared_system_memory <=> SharedSystemMemory,
        adapter_luid <=> AdapterLuid,
    }
}

impl From<DXGI_ADAPTER_DESC> for AdapterDesc {
    fn from(desc: DXGI_ADAPTER_DESC) -> AdapterDesc {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<AdapterDesc> for DXGI_ADAPTER_DESC {
    fn from(desc: AdapterDesc) -> DXGI_ADAPTER_DESC {
        unsafe { std::mem::transmute(desc) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AdapterDesc1 {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub subsys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: i64,
    pub flags: AdapterFlags,
}

impl AdapterDesc1 {
    pub fn description(&self) -> String {
        let len = wstrlens(&self.description);
        String::from_utf16_lossy(&self.description[..len])
    }
}

impl std::fmt::Debug for AdapterDesc1 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AdapterDesc1")
            .field("description", &self.description())
            .field("vendor_id", &self.vendor_id)
            .field("device_id", &self.device_id)
            .field("subsys_id", &self.subsys_id)
            .field("revision", &self.revision)
            .field("dedicated_video_memory", &self.dedicated_video_memory)
            .field("dedicated_system_memory", &self.dedicated_system_memory)
            .field("shared_system_memory", &self.shared_system_memory)
            .field("adapter_luid", &self.adapter_luid)
            .field("flags", &self.flags)
            .finish()
    }
}

#[cfg(test)]
member_compat_test! {
    adapter_desc1_compat:
    AdapterDesc1 <=> DXGI_ADAPTER_DESC1 {
        description <=> Description,
        vendor_id <=> VendorId,
        device_id <=> DeviceId,
        subsys_id <=> SubSysId,
        revision <=> Revision,
        dedicated_video_memory <=> DedicatedVideoMemory,
        dedicated_system_memory <=> DedicatedSystemMemory,
        shared_system_memory <=> SharedSystemMemory,
        adapter_luid <=> AdapterLuid,
        flags <=> Flags,
    }
}

impl From<DXGI_ADAPTER_DESC1> for AdapterDesc1 {
    fn from(desc: DXGI_ADAPTER_DESC1) -> AdapterDesc1 {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<AdapterDesc1> for DXGI_ADAPTER_DESC1 {
    fn from(desc: AdapterDesc1) -> DXGI_ADAPTER_DESC1 {
        unsafe { std::mem::transmute(desc) }
    }
}

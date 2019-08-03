use crate::descriptions::luid::Luid;
use crate::enums::{AdapterFlags, ComputePreemptionGranularity, GraphicsPreemptionGranularity};
use crate::helpers::wstrlens;
use crate::helpers::MemoryDbgHelper;

use checked_enum::UncheckedEnum;
use winapi::shared::dxgi::DXGI_ADAPTER_DESC;
use winapi::shared::dxgi::DXGI_ADAPTER_DESC1;
use winapi::shared::dxgi1_2::DXGI_ADAPTER_DESC2;
use winapi::shared::dxgi1_6::DXGI_ADAPTER_DESC3;

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
    pub adapter_luid: Luid,
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
            .field(
                "dedicated_video_memory",
                &MemoryDbgHelper(self.dedicated_video_memory as u64),
            )
            .field(
                "dedicated_system_memory",
                &MemoryDbgHelper(self.dedicated_system_memory as u64),
            )
            .field(
                "shared_system_memory",
                &MemoryDbgHelper(self.shared_system_memory as u64),
            )
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
    pub adapter_luid: Luid,
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
            .field(
                "dedicated_video_memory",
                &MemoryDbgHelper(self.dedicated_video_memory as u64),
            )
            .field(
                "dedicated_system_memory",
                &MemoryDbgHelper(self.dedicated_system_memory as u64),
            )
            .field(
                "shared_system_memory",
                &MemoryDbgHelper(self.shared_system_memory as u64),
            )
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AdapterDesc2 {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub subsys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: Luid,
    pub flags: AdapterFlags,
    pub graphics_preemption_granularity: UncheckedEnum<GraphicsPreemptionGranularity>,
    pub compute_preemption_granularity: UncheckedEnum<ComputePreemptionGranularity>,
}

impl AdapterDesc2 {
    pub fn description(&self) -> String {
        let len = wstrlens(&self.description);
        String::from_utf16_lossy(&self.description[..len])
    }
}

impl std::fmt::Debug for AdapterDesc2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AdapterDesc2")
            .field("description", &self.description())
            .field("vendor_id", &self.vendor_id)
            .field("device_id", &self.device_id)
            .field("subsys_id", &self.subsys_id)
            .field("revision", &self.revision)
            .field(
                "dedicated_video_memory",
                &MemoryDbgHelper(self.dedicated_video_memory as u64),
            )
            .field(
                "dedicated_system_memory",
                &MemoryDbgHelper(self.dedicated_system_memory as u64),
            )
            .field(
                "shared_system_memory",
                &MemoryDbgHelper(self.shared_system_memory as u64),
            )
            .field("adapter_luid", &self.adapter_luid)
            .field("flags", &self.flags)
            .field(
                "graphics_preemption_granularity",
                &self.graphics_preemption_granularity,
            )
            .field(
                "compute_preemption_granularity",
                &self.compute_preemption_granularity,
            )
            .finish()
    }
}

#[cfg(test)]
member_compat_test! {
    adapter_desc2_compat:
    AdapterDesc2 <=> DXGI_ADAPTER_DESC2 {
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
        graphics_preemption_granularity <=> GraphicsPreemptionGranularity,
        compute_preemption_granularity <=> ComputePreemptionGranularity,
    }
}

impl From<DXGI_ADAPTER_DESC2> for AdapterDesc2 {
    fn from(desc: DXGI_ADAPTER_DESC2) -> AdapterDesc2 {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<AdapterDesc2> for DXGI_ADAPTER_DESC2 {
    fn from(desc: AdapterDesc2) -> DXGI_ADAPTER_DESC2 {
        unsafe { std::mem::transmute(desc) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AdapterDesc3 {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub subsys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: Luid,
    pub flags: AdapterFlags,
    pub graphics_preemption_granularity: UncheckedEnum<GraphicsPreemptionGranularity>,
    pub compute_preemption_granularity: UncheckedEnum<ComputePreemptionGranularity>,
}

impl AdapterDesc3 {
    pub fn description(&self) -> String {
        let len = wstrlens(&self.description);
        String::from_utf16_lossy(&self.description[..len])
    }
}

impl std::fmt::Debug for AdapterDesc3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AdapterDesc3")
            .field("description", &self.description())
            .field("vendor_id", &self.vendor_id)
            .field("device_id", &self.device_id)
            .field("subsys_id", &self.subsys_id)
            .field("revision", &self.revision)
            .field(
                "dedicated_video_memory",
                &MemoryDbgHelper(self.dedicated_video_memory as u64),
            )
            .field(
                "dedicated_system_memory",
                &MemoryDbgHelper(self.dedicated_system_memory as u64),
            )
            .field(
                "shared_system_memory",
                &MemoryDbgHelper(self.shared_system_memory as u64),
            )
            .field("adapter_luid", &self.adapter_luid)
            .field("flags", &self.flags)
            .field(
                "graphics_preemption_granularity",
                &self.graphics_preemption_granularity,
            )
            .field(
                "compute_preemption_granularity",
                &self.compute_preemption_granularity,
            )
            .finish()
    }
}

#[cfg(test)]
member_compat_test! {
    adapter_desc3_compat:
    AdapterDesc3 <=> DXGI_ADAPTER_DESC3 {
        description <=> Description,
        vendor_id <=> VendorID,
        device_id <=> DeviceID,
        subsys_id <=> SubSysID,
        revision <=> Revision,
        dedicated_video_memory <=> DedicatedVideoMemory,
        dedicated_system_memory <=> DedicatedSystemMemory,
        shared_system_memory <=> SharedSystemMemory,
        adapter_luid <=> AdapterLuid,
        flags <=> Flags,
        graphics_preemption_granularity <=> GraphicsPreemptionGranularity,
        compute_preemption_granularity <=> ComputePreemptionGranularity,
    }
}

impl From<DXGI_ADAPTER_DESC3> for AdapterDesc3 {
    fn from(desc: DXGI_ADAPTER_DESC3) -> AdapterDesc3 {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<AdapterDesc3> for DXGI_ADAPTER_DESC3 {
    fn from(desc: AdapterDesc3) -> DXGI_ADAPTER_DESC3 {
        unsafe { std::mem::transmute(desc) }
    }
}

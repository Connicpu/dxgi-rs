use winapi::shared::dxgi::DXGI_FRAME_STATISTICS;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatistics {
    pub present_count: u32,
    pub present_refresh_count: u32,
    pub sync_refresh_count: u32,
    pub sync_qpc_time: i64,
    pub sync_gpu_time: i64,
}

#[cfg(test)]
member_compat_test! {
    frame_statistics_compat:
    FrameStatistics <=> DXGI_FRAME_STATISTICS {
        present_count <=> PresentCount,
        present_refresh_count <=> PresentRefreshCount,
        sync_refresh_count <=> SyncRefreshCount,
        sync_qpc_time <=> SyncQPCTime,
        sync_gpu_time <=> SyncGPUTime,
    }
}

impl From<DXGI_FRAME_STATISTICS> for FrameStatistics {
    fn from(desc: DXGI_FRAME_STATISTICS) -> FrameStatistics {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<FrameStatistics> for DXGI_FRAME_STATISTICS {
    fn from(desc: FrameStatistics) -> DXGI_FRAME_STATISTICS {
        unsafe { std::mem::transmute(desc) }
    }
}

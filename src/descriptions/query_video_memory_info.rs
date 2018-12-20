use winapi::shared::dxgi1_4::DXGI_QUERY_VIDEO_MEMORY_INFO;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct QueryVideoMemoryInfo {
    pub budget: u64,
    pub current_usage: u64,
    pub available_for_reservation: u64,
    pub current_reservation: u64,
}

#[cfg(test)]
member_compat_test! {
    swap_chain_desc_compat:
    QueryVideoMemoryInfo <=> DXGI_QUERY_VIDEO_MEMORY_INFO {
        budget <=> Budget,
        current_usage <=> CurrentUsage,
        available_for_reservation <=> AvailableForReservation,
        current_reservation <=> CurrentReservation,
    }
}

impl From<DXGI_QUERY_VIDEO_MEMORY_INFO> for QueryVideoMemoryInfo {
    fn from(desc: DXGI_QUERY_VIDEO_MEMORY_INFO) -> QueryVideoMemoryInfo {
        unsafe { std::mem::transmute(desc) }
    }
}

impl From<QueryVideoMemoryInfo> for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn from(desc: QueryVideoMemoryInfo) -> DXGI_QUERY_VIDEO_MEMORY_INFO {
        unsafe { std::mem::transmute(desc) }
    }
}

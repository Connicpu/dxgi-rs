flags! {
    #[repr(u32)]
    pub enum SwapChainFlags {
        NONPREROTATED = 1,
        ALLOW_MODE_SWITCH = 2,
        GDI_COMPATIBLE = 4,
        RESTRICTED_CONTENT = 8,
        RESTRICT_SHARED_RESOURCE_DRIVER = 16,
        DISPLAY_ONLY = 32,
        FRAME_LATENCY_WAITABLE_OBJECT = 64,
        FOREGROUND_LAYER = 128,
        FULLSCREEN_VIDEO = 256,
        YUV_VIDEO = 1024,
        ALLOW_TEARING = 2048,
    }
}

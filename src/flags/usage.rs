flags! {
    #[repr(u32)]
    pub enum UsageFlags {
        BACK_BUFFER = 1 << (2 + 4),
        DISCARD_ON_PRESENT = 1 << (5 + 4),
        READ_ONLY = 1 << (4 + 4),
        RENDER_TARGET_OUTPUT = 1 << (1 + 4),
        SHADER_INPUT = 1 << (0 + 4),
        SHARED = 1 << (3 + 4),
        UNORDERED_ACCESS = 1 << (6 + 4),
    }
}

flags! {
    #[repr(u32)]
    pub enum PresentFlags {
        TEST = 0x001,
        DO_NOT_SEQUENCE = 0x002,
        RESTART = 0x004,
        DO_NOT_WAIT = 0x008,
        RESTRICT_TO_OUTPUT = 0x010,
        STEREO_PREFER_RIGHT = 0x020,
        STEREO_TEMPORARY_MONO = 0x040,
        USE_DURATION = 0x100,
        ALLOW_TEARING = 0x200,
    }
}

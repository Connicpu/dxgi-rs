#[auto_enum::auto_enum(u32, checked)]
pub enum GraphicsPreemptionGranularity {
    DmaBufferBoundary = 0,
    PrimitiveBoundary = 1,
    TriangleBoundary = 2,
    PixelBoundary = 3,
    InstructionBoundary = 4,
}

#[auto_enum::auto_enum(u32, checked)]
pub enum ComputePreemptionGranularity {
    DmaBufferBoundary = 0,
    DispatchBoundary = 1,
    ThreadGroupBoundary = 2,
    ThreadBoundary = 3,
    InstructionBoundary = 4,
}

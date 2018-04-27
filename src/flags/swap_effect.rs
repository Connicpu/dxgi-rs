enum_! {
    #[repr(u32)]
    pub enum SwapEffect {
        Discard = 0,
        Sequential = 1,
        FlipSequential = 3,
        FlipDiscard = 4,
    }
}

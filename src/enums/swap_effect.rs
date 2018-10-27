#[auto_enum(u32, checked)]
pub enum SwapEffect {
    Discard = 0,
    Sequential = 1,
    FlipSequential = 3,
    FlipDiscard = 4,
}

#[auto_enum(u32, checked)]
pub enum ModeScanlineOrder {
    Unspecified = 0,
    Progressive = 1,
    UpperFieldFirst = 2,
    LowerFieldFirst = 3,
}

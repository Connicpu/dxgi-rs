#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Ratio {
    pub numerator: u32,
    pub denominator: u32,
}

impl Ratio {
    pub fn new(num: u32, den: u32) -> Self {
        Ratio {
            numerator: num,
            denominator: den,
        }
    }
}

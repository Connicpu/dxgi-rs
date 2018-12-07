use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The relative priority of a memory resource, which is often used to determine
/// which resources should be cleared to make room for others.
pub struct ResourcePriority(pub u32);

impl ResourcePriority {
    const MINIMUM: ResourcePriority = ResourcePriority(0x28000000);
    const LOW: ResourcePriority = ResourcePriority(0x50000000);
    const NORMAL: ResourcePriority = ResourcePriority(0x78000000);
    const HIGH: ResourcePriority = ResourcePriority(0xa0000000);
    const MAXIMUM: ResourcePriority = ResourcePriority(0xc8000000);
}

impl Add<u32> for ResourcePriority {
    type Output = Self;
    fn add(self, rhs: u32) -> Self {
        ResourcePriority(self.0 + rhs)
    }
}

impl Sub<u32> for ResourcePriority {
    type Output = Self;
    fn sub(self, rhs: u32) -> Self {
        ResourcePriority(self.0 - rhs)
    }
}

impl std::fmt::Debug for ResourcePriority {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::cmp::Ordering::*;
        static PREDEFINED: &[(ResourcePriority, &str)] = &[
            (ResourcePriority::MINIMUM, "MINIMUM"),
            (ResourcePriority::LOW, "LOW"),
            (ResourcePriority::NORMAL, "NORMAL"),
            (ResourcePriority::HIGH, "HIGH"),
            (ResourcePriority::MAXIMUM, "MAXIMUM"),
        ];

        let value = self.0 as i64;
        let (_, nearest, name) = PREDEFINED
            .iter()
            .map(|(p, name)| ((p.0 as i64 - value).abs(), p.0 as i64, name))
            .min()
            .unwrap();

        match value.cmp(&nearest) {
            Equal => write!(fmt, "ResourcePriority({})", name),
            Greater => write!(fmt, "ResourcePriority({} + {})", name, value - nearest),
            Less => write!(fmt, "ResourcePriority({} + {})", name, value + nearest),
        }
    }
}

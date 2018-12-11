use crate::factory::Factory1;
use crate::adapter::Adapter1;

#[derive(Copy, Clone)]
/// An iterator over the graphics adapters on the computer.
pub struct AdapterIter1<'a> {
    pub(super) factory: &'a Factory1,
    pub(super) adapter: u32,
}

impl<'a> Iterator for AdapterIter1<'a> {
    type Item = Adapter1;

    fn next(&mut self) -> Option<Adapter1> {
        let result = self.factory.enum_adapter(self.adapter);
        self.adapter += 1;
        result
    }
}

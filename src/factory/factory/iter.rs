use crate::factory::Factory;
use crate::adapter::Adapter;

#[derive(Copy, Clone)]
/// An iterator over the graphics adapters on the computer.
pub struct AdapterIter<'a> {
    pub(super) factory: &'a Factory,
    pub(super) adapter: u32,
}

impl<'a> Iterator for AdapterIter<'a> {
    type Item = Adapter;

    fn next(&mut self) -> Option<Adapter> {
        let result = self.factory.enum_adapter(self.adapter);
        self.adapter += 1;
        result
    }
}

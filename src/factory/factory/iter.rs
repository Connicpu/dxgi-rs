use crate::adapter::Adapter;
use crate::factory::IFactory;

#[derive(Copy, Clone)]
/// An iterator over the graphics adapters on the computer.
pub struct AdapterIter<'a, F: IFactory + ?Sized> {
    pub(super) factory: &'a F,
    pub(super) adapter: u32,
}

impl<'a, F: IFactory + ?Sized> Iterator for AdapterIter<'a, F> {
    type Item = Adapter;

    fn next(&mut self) -> Option<Adapter> {
        let result = self.factory.enum_adapter(self.adapter);
        self.adapter += 1;
        result
    }
}

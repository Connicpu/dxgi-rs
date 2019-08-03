use crate::adapter::Adapter1;
use crate::factory::IFactory1;

#[derive(Copy, Clone)]
/// An iterator over the graphics adapters on the computer.
pub struct AdapterIter1<'a, F: IFactory1 + ?Sized> {
    pub(super) factory: &'a F,
    pub(super) adapter: u32,
}

impl<'a, F: IFactory1 + ?Sized> Iterator for AdapterIter1<'a, F> {
    type Item = Adapter1;

    fn next(&mut self) -> Option<Adapter1> {
        let result = self.factory.enum_adapter1(self.adapter);
        self.adapter += 1;
        result
    }
}

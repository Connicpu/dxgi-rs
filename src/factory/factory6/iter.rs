use crate::adapter::AdapterType;
use crate::enums::GpuPreference;
use dcommon::error::Error;
use crate::factory::factory6::Factory6;

use std::marker::PhantomData;

/// An iterator over the graphics adapters on the computer.
/// 
/// See [`Factory6::adapters_by_preference`][1]
/// 
/// [1]: struct.Factory6.html#method.adapters_by_preference
pub struct AdapterIterByPreference<'a, A: AdapterType> {
    pub(super) factory: &'a Factory6,
    pub(super) adapter: u32,
    pub(super) preference: GpuPreference,
    pub(super) _marker: PhantomData<A>,
}

impl<'a, A: AdapterType> Copy for AdapterIterByPreference<'a, A> {}
impl<'a, A: AdapterType> Clone for AdapterIterByPreference<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, A> Iterator for AdapterIterByPreference<'a, A>
where
    A: AdapterType,
{
    type Item = Result<A, Error>;

    fn next(&mut self) -> Option<Result<A, Error>> {
        let result = self
            .factory
            .enum_adapter_by_preference::<A>(self.adapter, self.preference);
        self.adapter += 1;
        result
    }
}

impl<'a, A> std::fmt::Debug for AdapterIterByPreference<'a, A>
where
    A: AdapterType + std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_list().entries(*self).finish()
    }
}

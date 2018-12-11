use com_wrapper::ComWrapper;

pub use self::adapter::Adapter;
pub use self::adapter1::Adapter1;
pub use self::adapter2::Adapter2;
pub use self::adapter3::Adapter3;
pub use self::adapter4::Adapter4;

pub mod adapter;
pub mod adapter1;
pub mod adapter2;
pub mod adapter3;
pub mod adapter4;

pub trait AdapterType: ComWrapper + Clone {
    /// Try to cast this adapter to a different adapter type
    fn try_cast<A: AdapterType>(&self) -> Option<A> {
        unsafe {
            let ptr = self.clone().into_ptr();
            Some(ComWrapper::from_ptr(ptr.cast().ok()?))
        }
    }
}

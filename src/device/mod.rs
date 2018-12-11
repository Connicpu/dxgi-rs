use com_wrapper::ComWrapper;

pub use self::device::Device;
pub use self::device1::Device1;
pub use self::device2::Device2;
pub use self::device3::Device3;
pub use self::device4::Device4;

pub mod device;
pub mod device1;
pub mod device2;
pub mod device3;
pub mod device4;

pub trait DeviceType: ComWrapper + Clone {
    /// Try to cast this factory to a different factory type
    fn try_cast<D: DeviceType>(&self) -> Option<D> {
        unsafe {
            let ptr = self.clone().into_ptr();
            Some(ComWrapper::from_ptr(ptr.cast().ok()?))
        }
    }
}

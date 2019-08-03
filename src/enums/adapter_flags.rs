#[auto_enum::enum_flags(u32)]
/// Identifies the type of DXGI adapter.
pub enum AdapterFlags {
    /// This flag is reserved.
    REMOTE = 1,

    /// Specifies a software adapter. For more info about this flag, see new
    /// info in Windows 8 about enumerating adapters.
    ///
    /// **Direct3D 11:**
    /// This enumeration value is supported starting with Windows 8.
    SOFTWARE = 2,
}

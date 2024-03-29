use crate::bindings::NvU32;

pub const fn make_nvapi_version<T>(version: u32) -> NvU32 {
    std::mem::size_of::<T>() as u32 | version << 16
}

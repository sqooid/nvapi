#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn make_nvapi_version<T>(version: u32) -> NvU32 {
    std::mem::size_of::<T>() as u32 | version << 16
}

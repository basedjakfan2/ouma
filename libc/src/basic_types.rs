#![allow(non_camel_case_types)]

pub type c_char = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

// platform specific types
#[cfg(target_arch = "x86")]
pub type wchar_t = crate::arch::x86::wchar_t;

#[cfg(target_arch = "x86_64")]
pub type wchar_t = crate::arch::x86_64::wchar_t;

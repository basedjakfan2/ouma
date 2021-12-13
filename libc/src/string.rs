use core::ffi::c_void;

use crate::basic_types::c_int;
use crate::basic_types::c_char;
use crate::basic_types::size_t;

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
	return memmove(dest, src, n);
}

#[no_mangle]
pub extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
	let dest1: *mut c_char = dest as *mut c_char;
	let src1: *const c_char = src as *const c_char;

	if (dest1 as *const c_char) < src1 {
		let mut i = 0;

		while i < n {
			unsafe { *(dest1).add(i) = *(src1).add(i); }
			i += 1;
		}
	} else if (dest1 as *const c_char) > src1 {
		let mut i = n;

		while i > 0 {
			i -= 1;
			unsafe { *(dest1).add(i) = *(src1).add(i); }
		}
	}

	return dest;
}

#[no_mangle]
pub extern "C" fn ata_memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
	let dest1: *mut c_char = dest as *mut c_char;
	let mut i = n;

	while i > 0 {
		i -= 1;
		unsafe { *(dest1).add(i) = c as c_char; }
	}

	return dest;
}

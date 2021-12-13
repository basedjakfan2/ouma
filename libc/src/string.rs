use core::ffi::c_void;
use core::ptr::null_mut;

use crate::basic_types::c_int;
use crate::basic_types::c_char;
use crate::basic_types::c_uchar;
use crate::basic_types::size_t;

#[no_mangle]
pub extern "C" fn memchr(src: *const c_void, c: c_int, n: size_t) -> *mut c_void {
	let src1: *const c_char = src as *const c_char;
	let mut i = n;

	unsafe {
		while i > 0 {
			i -= 1;
			if *src1.add(i) == (c as c_char) {
				return src1.add(i) as *mut c_void;
			}
		}
	}

	return null_mut();
}

#[no_mangle]
pub extern "C" fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int {
	let cx1: *const c_uchar = cx as *const c_uchar;
	let ct1: *const c_uchar = ct as *const c_uchar;
	let mut i = n;

	while i > 0 {
		i -= 1;
		unsafe {
			let cx2: c_uchar = *(cx1).add(i);
			let ct2: c_uchar = *(ct1).add(i);

			if cx2 != ct2 {
				return (cx2 - ct2).into();
			}
		}
	}

	return 0;
}

// Adopted from Redox OS
#[no_mangle]
pub extern "C" fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void {
	let to = memchr(src, c, n);
	if to.is_null() {
		return to;
	}

	let dist = (to as usize) - (src as usize);
	if memcpy(dest, src, dist).is_null() {
		return null_mut();
	}

	unsafe { return (dest as *mut u8).add(dist + 1) as *mut c_void; }
}

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
pub extern "C" fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
	let dest1: *mut c_char = dest as *mut c_char;
	let mut i = n;

	while i > 0 {
		i -= 1;
		unsafe { *(dest1).add(i) = c as c_char; }
	}

	return dest;
}

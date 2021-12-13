use core::ptr::null_mut;

use crate::basic_types::c_int;
use crate::basic_types::size_t;
use crate::basic_types::wchar_t;

#[no_mangle]
pub extern "C" fn wmemchr(src: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t {
	let mut i = n;

	unsafe {
		while i > 0 {
			i -= 1;
			if *src.add(i) == c {
				return src.add(i) as *mut wchar_t;
			}
		}
	}

	return null_mut();
}

#[no_mangle]
pub extern "C" fn wmemcmp(cx: *const wchar_t, ct: *const wchar_t, n: size_t) -> c_int {
	let cx1: *const wchar_t = cx as *const wchar_t;
	let ct1: *const wchar_t = ct as *const wchar_t;
	let mut i = n;

	while i > 0 {
		i -= 1;
		unsafe {
			let cx2: wchar_t = *(cx1).add(i);
			let ct2: wchar_t = *(ct1).add(i);

			if cx2 != ct2 {
				return (cx2 - ct2).into();
			}
		}
	}

	return 0;
}

#[no_mangle]
pub extern "C" fn wmemcpy(dest: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t {
	return wmemmove(dest, src, n);
}

#[no_mangle]
pub extern "C" fn wmemmove(dest: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t {
	let dest1: *mut wchar_t = dest;
	let src1: *const wchar_t = src;

	if (dest1 as *const wchar_t) < src1 {
		let mut i = 0;

		while i < n {
			unsafe { *(dest1).add(i) = *(src1).add(i); }
			i += 1;
		}
	} else if (dest1 as *const wchar_t) > src1 {
		let mut i = n;

		while i > 0 {
			i -= 1;
			unsafe { *(dest1).add(i) = *(src1).add(i); }
		}
	}

	return dest;
}

#[no_mangle]
pub extern "C" fn wmemset(dest: *mut wchar_t, c: c_int, n: size_t) -> *mut wchar_t {
	let dest1: *mut wchar_t = dest as *mut wchar_t;
	let mut i = n;

	while i > 0 {
		i -= 1;
		unsafe { *(dest1).add(i) = c as wchar_t; }
	}

	return dest;
}

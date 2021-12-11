use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
	let dest1: *mut i8 = dest as *mut i8;
	let src1: *const i8 = src as *const i8;

	if (dest1 as *const i8) < src1 {
		let mut i = 0;

		while i < n {
			unsafe { *(dest1).add(i) = *(src1).add(i); }
			i += 1;
		}
	} else if (dest1 as *const i8) > src1 {
		let mut i = n;

		while i > 0 {
			i -= 1;
			unsafe { *(dest1).add(i) = *(src1).add(i); }
		}
	}

	return dest;
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
	return memmove(dest, src, n);
}

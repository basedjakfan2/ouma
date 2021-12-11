#[no_mangle]
pub extern "C" fn wmemmove(dest: *mut i32, src: *const i32, n: usize) -> *mut i32 {
	let dest1: *mut i32 = dest;
	let src1: *const i32 = src;

	if (dest1 as *const i32) < src1 {
		let mut i = 0;

		while i < n {
			unsafe { *(dest1).add(i) = *(src1).add(i); }
			i += 1;
		}
	} else if (dest1 as *const i32) > src1 {
		let mut i = n;

		while i > 0 {
			i -= 1;
			unsafe { *(dest1).add(i) = *(src1).add(i); }
		}
	}

	return dest;
}

#[no_mangle]
pub extern "C" fn wmemcpy(dest: *mut i32, src: *const i32, n: usize) -> *mut i32 {
	return wmemmove(dest, src, n);
}

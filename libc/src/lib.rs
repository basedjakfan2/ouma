#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
        loop {}
}

pub mod basic_types;

pub mod arch;

pub mod string;
pub mod wchar;

// https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
// https://doc.rust-lang.org/nomicon/beneath-std.html

#![no_main]
#![no_std]

use core::ffi::{c_char, c_int};

unsafe extern "C" {
   fn c_main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[unsafe(no_mangle)]
fn main(argc: c_int, argv: *const *const c_char) -> c_int {
   unsafe { c_main(argc, argv) }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
   loop {}
}

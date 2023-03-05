#![no_std]
#![no_main]

use libc::{c_char, c_int};

#[link(name = "c")]
extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(code: c_int) -> !;
}

include!(concat!(env!("OUT_DIR"), "/doggo.rs"));

#[no_mangle]
extern "C" fn main() -> isize {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_panic: &PanicInfo<'_>) -> ! {
        unsafe {
            printf("rust panicked. git gud\n\0".as_ptr() as *const c_char);
            exit(1);
        }
    }

    doggo();

    0
}

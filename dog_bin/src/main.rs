#![no_std]
#![no_main]

#[link(name = "c")]
extern "C" {
    fn printf(format: *const i8, ...) -> isize;
    fn exit(code: i8) -> !;
}

include!(concat!(env!("OUT_DIR"), "/doggo.rs"));

#[no_mangle]
extern "C" fn main() -> isize {
    doggo();

    0
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    unsafe {
        printf("rust panicked. git gud\n\0".as_ptr().cast::<i8>());
        exit(1);
    }
}

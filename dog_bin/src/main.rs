#![no_std]
#![no_main]

#[link(name = "c")]
extern "C" {
    fn printf(format: *const i8, ...) -> isize;
}

include!(concat!(env!("OUT_DIR"), "/doggo.rs"));

#[no_mangle]
extern "C" fn main() -> isize {
    doggo();

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    #[link(name = "c")]
    extern "C" {
        fn exit(code: i8) -> !;
    }
    unsafe {
        printf("rust panicked. git gud\n\0".as_ptr().cast::<i8>());
        exit(1);
    }
}

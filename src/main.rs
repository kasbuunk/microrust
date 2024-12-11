#![no_std]
#![no_main]

extern crate panic_halt;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

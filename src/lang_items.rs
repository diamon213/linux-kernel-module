//! Certain language items needed with the usage of `#![no_std]`
use core;

#[no_mangle]
#[lang = "eh_personality"]

pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
#[lang = "eh_unwind_resume"]

pub extern "C" fn rust_eh_unwind_resume() {}

#[no_mangle]
#[lang = "panic_impl"]

pub extern "C" fn rust_begin_panic(_info: &core::panic::PanicInfo) -> ! {
    let _ = *(core::ptr::null::<i32>());
    // infinite loop
    loop {}
}
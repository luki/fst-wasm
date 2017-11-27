#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub fn factorial(n: i32) -> i32 {
    if n == 1 { return 1 }
    n * factorial(n-1)
}


// needed for no_std
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32,
                               _column: u32) -> ! {
                                   loop {}
}

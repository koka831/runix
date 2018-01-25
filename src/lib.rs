#![feature(lang_items)]
#![no_std]

extern crate rlibc;
#[no_mangle]
pub extern fn boot() {
    let hello = b"Hello, World";
    let color = 0x1f; // f:white b:blue
    let mut hello_clr = [color; 24];
    for (i, chr) in hello.into_iter().enumerate() {
        hello_clr[i * 2] = *chr;
    }

    let buf_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buf_ptr = hello_clr };

    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }

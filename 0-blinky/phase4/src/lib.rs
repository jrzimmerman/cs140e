#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // STEP 1: Set GPIO Pin 16 as output.
    let fsel1 = GPIO_FSEL1.read_volatile();
    // left shift gpio pin register multiplied by
    // register offset for raspberry pi
    GPIO_FSEL1.write_volatile(fsel1 | 1 << ((16 % 10) * 3));
    // STEP 2: Continuously set and clear GPIO 16.
    loop {
        GPIO_SET0.write_volatile(1 << 16);
        spin_sleep_ms(100);
        GPIO_CLR0.write_volatile(1 << 16);
        spin_sleep_ms(100);
    }
}

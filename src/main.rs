#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;

extern "C" {
    fn wfi();
}

#[entry]
fn main() -> ! {
    unsafe {
        wfi();
        wfi();
        wfi();
        wfi();
        wfi();
    };

    loop {
        // your code goes here
    }
}

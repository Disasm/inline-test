#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;



mod asm {
    #[inline(always)]
    pub fn wfi() {
        extern "C" {
            fn wfi_c();
        }
        unsafe {
            wfi_c();
        }
    }
}

#[entry]
fn main() -> ! {
    use asm::wfi;

    wfi();
    wfi();
    wfi();
    wfi();
    wfi();

    loop {
        // your code goes here
    }
}

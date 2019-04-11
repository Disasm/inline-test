__attribute__((always_inline))
void wfi_c() {
    __asm__("wfi");
}


global_asm!(include_str!("boot.asm"));

fn boot_init() {
    //TODO: zero bss
}

#[no_mangle]
extern "C" fn rust_boot(hartid: usize, _fdt: *const u8) -> ! {
    if hartid == 0 {
        boot_init();
    }

    crate::smp_main(hartid)
}
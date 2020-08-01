#![no_std]
#![no_main]

#![feature(global_asm)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
mod uart;
mod arch;

use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" fn user_fn(a0: usize) {
    println!("Hello user {}", a0);
    unsafe { riscv::asm::ebreak(); }
}

fn single_main() {
    println!("Hello World!");

    unsafe { arch::trap::init(); }

    let mut user_stack = [0 as usize; 512];

    let mut task0 = arch::trap::Context::default();
    task0.set_status(0x80); // Machine previous interrupt enable
    task0.set_epc(user_fn as _);
    task0.set_x(2, &mut user_stack[511] as *mut _ as _);

    println!("Starting user level task");
    unsafe { task0.run(); }

    println!("Survived user level");
}

fn smp_main(cpu_id: usize) -> ! {
    if cpu_id == 0 {
        single_main();
    }

    loop{}
}
use core::panic::PanicInfo;

use crate::println;

pub fn runner(tests: &[&dyn Fn()]) {
    crate::init();
    
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    success();
}

pub fn default_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    fail();
    loop {}
}

fn exit_qemu(exit_code: u32) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }
}

pub fn fail() {
    exit_qemu(0x11);
}

pub fn success() {
    exit_qemu(0x10);
}
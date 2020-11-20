#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pawos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pawos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    pawos::init();
    x86_64::instructions::interrupts::int3();

    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pawos::test_panic_handler(info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
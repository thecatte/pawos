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

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    pawos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pawos::hlt_loop();
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
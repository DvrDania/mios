#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Don't forget to add a loop if there is no panic
    panic!("Panic message :(");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

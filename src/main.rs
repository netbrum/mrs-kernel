#![no_std]
#![no_main]

use vga::WRITER;

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..=50 {
        println!("{}", i);
    }

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    WRITER.lock().reset();
    println!("{}", info);

    loop {}
}

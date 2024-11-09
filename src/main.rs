#![no_std]
#![no_main]

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXXX");
    println!("XXXXXXXXXXXXX");
    println!("XXXXXXXXXXXX");
    println!("XXXXXXXXXXX");
    println!("XXXXXXXXXX");
    println!("XXXXXXXXX");
    println!("XXXXXXXX");
    println!("XXXXXXX");
    println!("XXXXXX");
    println!("XXXXX");
    println!("XXXX");
    println!("XXX");
    println!("XX");
    print!("X");

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga::WRITER.lock().reset();
    println!("{}", info);

    loop {}
}

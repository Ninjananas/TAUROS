#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;
/// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}{}", vga_buffer::Color::LightRed as u8 as char, info);
    loop {}
}

// Beginning of kernel

#[macro_use]
mod vga_buffer;
mod multiboot;
mod interrupt;

extern "C" {
    static DIRECT_MAP_OFFSET: u64;
    static mb2_info_pa: u32;
//    fn rdrand() -> u64;
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {

    // #[cfg(test)]
    // test_main();

    // Get Multiboot2 Information
    let mb2_addr = unsafe{DIRECT_MAP_OFFSET as usize + mb2_info_pa as usize};

    let boot_info = multiboot::load(mb2_addr);
    println!("\nmemory size: {} MB", boot_info.total_memory_size() / 0x100000);


    interrupt::init_idt();


    // TODO : Create direct map at random location

    loop {}
}

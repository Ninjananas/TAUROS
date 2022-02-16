#![no_std]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(abi_x86_interrupt)]
// #![feature(const_fn)]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// #![reexport_test_harness_main = "test_main"]
// #![feature(lang_items)]

use core::panic::PanicInfo;
/// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}{}", vga_buffer::Color::LightRed as u8 as char, info);
    loop {}
}

// #[cfg(test)]
// fn test_runner(tests: & [&dyn Fn()]) {
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }
// }

// #[lang = "start"]
// fn tmain(argc: usize, argv: i64) -> usize {
//     0
// }

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

    println!("\nmemory size: {} MB", multiboot::get_memory_size(mb2_addr) / 0x100000);

    // TODO : Create direct map at random location

    loop {}
}

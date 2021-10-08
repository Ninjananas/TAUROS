#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RegionType {
    Available = 1,
    Reserved = 2,
    ACPI_Reclamable = 3,
    ACPI_NVS = 4,
    Bad = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct MemoryRegion {
    base_addr: u64,
    region_length: u64,
    region_type: RegionType,
    acpi_extended_attributes: u32,
}


pub fn detect_upper_memory() {
    unsafe {
        llvm_asm!("xorl %ebx, %ebx");
        llvm_asm!("movl 0x534D4150, %edx");
        llvm_asm!("movl 0xE820, %eax");
        llvm_asm!("movl 24, %ecx");
    };
}

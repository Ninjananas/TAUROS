use super::tags::Tag;

pub fn get_memory_size_from_memory_tag(tag: &Tag) -> u64 {
    let mut current_entry_addr = tag.address() + 16; // +16 to skip header
    let entry_size = unsafe{*((tag.address() + 8) as *const u32)};
    let mut remaining_bytes = tag.size - 16;
    let mut total_size: u64 = 0;
    while remaining_bytes != 0 {
        total_size += unsafe{*((current_entry_addr + 8) as *const u64)};
        current_entry_addr += entry_size as usize;
        remaining_bytes -= entry_size;
    }
    total_size
}

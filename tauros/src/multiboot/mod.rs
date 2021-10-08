mod tags;
use tags::{TagIter, Tag};

pub fn get_memory_size(multiboot_addr: usize) -> u64 {
    let mut cur_addr: usize = multiboot_addr + 8;
    let mut cur_typ: u32 = unsafe{*(cur_addr as *const u32)};
    let mut cur_size: u32 = unsafe{*((cur_addr + 4) as *const u32)};
    while cur_typ != 6 {
        if (cur_typ == 0) && (cur_size == 8) {
            panic!("No memory map information in multiboot header");
        }
        cur_addr += ((cur_size + 7) & !7) as usize;
        cur_typ = unsafe{*(cur_addr as *const u32)};
        cur_size = unsafe{*((cur_addr + 4) as *const u32)};
    }
    let entry_size: u32 = unsafe{*((cur_addr + 8) as *const u32)};
    cur_addr += 16 + 8; // 16 to skip tag header, 8 to target the'length' field of entry
    cur_size -= 16;
    let mut min_addr: u64 = 0xffffffffffffffff;
    let mut max_addr: u64 = 0;
    while cur_size != 0 {
        let sz: u64 = unsafe{*(cur_addr as *const u64)};
        let addr: u64 = unsafe{*((cur_addr-8) as *const u64)};
        if addr < min_addr {
            min_addr = addr;
        }
        if addr + sz > max_addr {
            max_addr = addr + sz;
        }
        cur_addr += entry_size as usize;
        cur_size -= entry_size;
    }
    max_addr - min_addr
}

#[repr(C, packed)]
struct MultibootHeader {
    total_size: u32,
    reserved: u32,
}

pub struct BootInformation {
    header: *const MultibootHeader,
    offset: usize,
}

pub fn load(address: usize) -> BootInformation {
    let header = unsafe{&*(address as *const MultibootHeader)};
    BootInformation{header: header, offset: 0}
}

impl BootInformation {

    pub fn start_address(&self) -> usize {
        self.header as usize
    }

    pub fn end_address(&self) -> usize {
        self.start_address() + self.total_size()
    }

    pub fn total_size(&self) -> usize {
        self.get_header().total_size as usize
    }

    fn get_header(&self) -> &MultibootHeader {
        unsafe { &*self.header }
    }

    fn get_tag<'a>(&'a self, typ:u32) -> Option<&'a Tag> {
        self.tags().find(|tag| tag.typ == typ)
    }

    fn tags(&self) -> TagIter {
        TagIter::new(unsafe {self.header.offset(1)} as *const _)
    }

    
}

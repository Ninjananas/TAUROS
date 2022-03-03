mod tags;
mod memory_map;
use tags::{TagIter, Tag, TagType};


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

    pub fn get_tag<'a>(&'a self, typ:TagType) -> Option<&'a Tag> {
        self.tags().find(|tag| tag.typ == typ)
    }

    pub fn tags(&self) -> TagIter {
        TagIter::new(unsafe {self.header.offset(1)} as *const _)
    }

    pub fn total_memory_size(&self) -> u64 {
        match self.get_tag(TagType::MemoryMap) {
            None => return 0,
            Some(tag) => memory_map::get_memory_size_from_memory_tag(tag),
        }
    }
}

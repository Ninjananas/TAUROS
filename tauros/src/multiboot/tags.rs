use core::marker::PhantomData;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[repr(u32)]
pub enum TagType {
    LastTag = 0,
    BootCommandLine = 1,
    BootLoaderName = 2,
    Modules = 3,
    MemoryMap = 6,
    VBEInfi = 7,
    FrameBufferInfo = 8,
    ELFSymbols = 9,
    APMTable = 10,
    EFI32SystemTablePointer = 11,
    EFI64SystemTablePointer = 12,
    SMBIOSTables = 13,
    ACPIoldRSDP = 14,
    ACPInewRSDP = 15,
    NetworkingInformation = 16,
    EFIMemoryMap = 17,
    EFIBootServicesNotTerminated = 18,
    EFI32ImageHandlePointer = 19,
    EFI64ImageHandlePointer = 20,
    ImageLoadBasePhysicalAddress = 21,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Tag {
    pub typ: TagType,
    pub size: u32,
}

impl Tag {
    pub fn address(&self) -> usize {
        (self as *const Self) as usize
    }
}

pub struct TagIter<'a> {
    pub current: *const Tag,
    phantom: PhantomData<&'a Tag>,
}

impl<'a> TagIter<'a> {
    pub fn new(first: *const Tag) -> TagIter<'a> {
        TagIter {
            current: first,
            phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = &'a Tag;

    fn next(&mut self) -> Option<&'a Tag> {
        match unsafe{&*self.current} {
            &Tag{typ: TagType::LastTag, size: 8} => None, // Last tag
            tag => {
                let mut tag_addr = self.current as usize;
                tag_addr += ((tag.size + 7) & !7) as usize; // align 8 bytes
                self.current = tag_addr as *const _;
                Some(tag)
            },
        }
    }
}

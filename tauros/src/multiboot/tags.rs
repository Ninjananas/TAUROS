use core::marker::PhantomData;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Tag {
    pub typ: u32,
    pub size: u32,
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
            &Tag{typ: 0, size: 8} => None, // Last tag
            tag => {
                let mut tag_addr = self.current as usize;
                tag_addr += ((tag.size + 7) & !7) as usize; // align 8 bytes
                self.current = tag_addr as *const _;
                Some(tag)
            },
        }
    }
}

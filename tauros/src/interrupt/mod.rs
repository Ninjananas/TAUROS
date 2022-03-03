use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::arch::asm;
use core::mem::size_of;

use lazy_static::lazy_static;

#[repr(C, packed)]
struct InterruptDescriptorTableDescriptor {
    size: u16,
    offset: u64,
}

#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    divide_by_zero: Entry<Gate>, // Vector 0
    debug: Entry<Gate>, // Vector 1
    non_maskable_interrupt: Entry<Gate>, // Vector 2
    breakpoint: Entry<Gate>, // Vector 3
    overflow: Entry<Gate>, // Vector 4
    bound_range_exceeded: Entry<Gate>, // Vector 5
    invalid_opcode: Entry<Gate>, // Vector 6
    device_not_available: Entry<Gate>, // Vector 7
    double_fault: Entry<GateWithErrorCode>, // Vector 8
    coprocessor_segment_overrun: Entry<Gate>, // Vector 9
    invalid_tss: Entry<GateWithErrorCode>, // Vector 10
    segment_not_present: Entry<GateWithErrorCode>, // Vector 11
    stack_segment_fault: Entry<GateWithErrorCode>, // Vector 12
    general_protection_fault: Entry<GateWithErrorCode>, // Vector 13
    page_fault: Entry<GateWithErrorCode>, // Vector 14
    reserved_15: Entry<Gate>, // Vector 15
    x87_floating_point_exception: Entry<Gate>, // Vector 16
    alignment_check: Entry<GateWithErrorCode>, // Vector 17
    machine_check: Entry<Gate>, // Vector 18
    simd_floating_point_exception: Entry<Gate>, // Vector 19
    virtualization_exception: Entry<Gate>, // Vector 20
    reserved_21_30: [Entry<Gate>; 9], // Vectors 21 to 29
    security_exception: Entry<GateWithErrorCode>, // Vector 30
    reserved_31: Entry<Gate>, // Vector 31

    interrupts: [Entry<Gate>; 256 - 32], // general interrupts
}

impl InterruptDescriptorTable {
    pub /*const*/ fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_by_zero: Entry::missing(),
            debug: Entry::missing(),
            non_maskable_interrupt: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            reserved_15: Entry::missing(),
            x87_floating_point_exception: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point_exception: Entry::missing(),
            virtualization_exception: Entry::missing(),
            reserved_21_30: [Entry::missing(); 9],
            security_exception: Entry::missing(),
            reserved_31: Entry::missing(),
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    // static lifetime, InterruptDescriptorTable will be used during the whole OS execution
    pub fn load(&'static self) {
        let idtr = InterruptDescriptorTableDescriptor {
            offset: self as *const _ as u64,
            size: (size_of::<Self>() - 1) as u16,
        };
        unsafe{ asm!("lidt ({0:r})", in(reg) &idtr as *const _ as u64 , options(att_syntax)) };
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = Entry<Gate>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.divide_by_zero,
            1 => &self.debug,
            2 => &self.non_maskable_interrupt,
            3 => &self.breakpoint,
            4 => &self.overflow,
            5 => &self.bound_range_exceeded,
            6 => &self.invalid_opcode,
            7 => &self.device_not_available,
            9 => &self.coprocessor_segment_overrun,
            16 => &self.x87_floating_point_exception,
            18 => &self.machine_check,
            19 => &self.simd_floating_point_exception,
            20 => &self.virtualization_exception,
            i @ 32..=255 => &self.interrupts[i - 32],
            i @ 15 | i @ 21..=29 | i @ 31 => panic!("InterruptDescriptorTable entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 30 => panic!("InterruptDescriptorTable entry {} has an error code and can't be accessed through an index", i),
            i => panic!("InterruptDescriptorTable entry {} does not exist", i),
        }
    }
}


impl IndexMut<usize> for InterruptDescriptorTable {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.divide_by_zero,
            1 => &mut self.debug,
            2 => &mut self.non_maskable_interrupt,
            3 => &mut self.breakpoint,
            4 => &mut self.overflow,
            5 => &mut self.bound_range_exceeded,
            6 => &mut self.invalid_opcode,
            7 => &mut self.device_not_available,
            9 => &mut self.coprocessor_segment_overrun,
            16 => &mut self.x87_floating_point_exception,
            18 => &mut self.machine_check,
            19 => &mut self.simd_floating_point_exception,
            20 => &mut self.virtualization_exception,
            i @ 32..=255 => &mut self.interrupts[i - 32],
            i @ 15 | i @ 21..=29 | i @ 31 => panic!("InterruptDescriptorTable entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 30 => panic!("InterruptDescriptorTable entry {} has an error code and can't be accessed through an index", i),
            i => panic!("InterruptDescriptorTable entry {} does not exist", i),
        }
    }
}


pub type Gate = extern "x86-interrupt" fn(&mut InterruptFrame);
pub type GateWithErrorCode = extern "x86-interrupt" fn(&mut InterruptFrame, error_code: u64);

#[repr(C)]
pub struct InterruptFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Entry<T> {
    pointer_low: u16,
    gdt_selector: u16,
    options: Options,
    pointer_mid: u16,
    pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<T>,
}

impl<T> Entry<T> {
    pub const fn missing() -> Self {
        Entry {
            pointer_low: 0,
            pointer_mid: 0,
            pointer_high: 0,
            gdt_selector: 0,
            options: Options::minimal(),
            reserved: 0,
            phantom: PhantomData,
        }
    }

    #[cfg(target_arch = "x86_64")]
    fn set_gate_addr(&mut self, addr: u64) -> &mut Self {
        self.pointer_low = addr as u16;
        self.pointer_mid = (addr >> 16) as u16;
        self.pointer_high = (addr >> 32) as u32;

        unsafe { asm!("movw %cs, {0:x}", out(reg) self.gdt_selector, options(att_syntax)) };

        self.options.set_present(true);

        self
    }
}

impl Entry<Gate> {
    pub fn set_gate_fn(&mut self, gate: Gate) -> &mut Self {
        self.set_gate_addr(gate as u64)
    }
}

impl Entry<GateWithErrorCode> {
    pub fn set_gate_fn(&mut self, gate: GateWithErrorCode) -> &mut Self {
        self.set_gate_addr(gate as u64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Options(u16);

macro_rules! set_bit {
    ($n:expr, $bit:expr, $val:expr) => {
        $n = ($n & !(1<<$bit)) | ($val as u16) << $bit;
    };
}

macro_rules! set_bits {
    ($n:expr, $startbit:expr, $endbit:expr, $val:expr) => {
        $n = ($n & !((1<<$endbit) - (1<<$startbit))) | (($val << $startbit) & ((1<<$endbit) - (1<<$startbit)));
    };
}

impl Options {
    const fn minimal() -> Self {
        Options((1<<9) | (1<<10) | (1<<11))
    }

    fn set_present(&mut self, present: bool) -> &mut Self {
        set_bit!(self.0, 15, present);
        self
    }

    fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        set_bit!(self.0, 8, !disable);
        self
    }

    fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        set_bits!(self.0, 13, 15, dpl);
        self
    }

    fn set_stack_index(&mut self, index: u16) -> &mut Self {
        set_bits!(self.0, 0, 3, index);
        self
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_gate_fn(breakpoint_gate);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_gate(stack_frame: &mut InterruptFrame) {
    println!("EXCEPTION: BREAKPOINT\n");
}

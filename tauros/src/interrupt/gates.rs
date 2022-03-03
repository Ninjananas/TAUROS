use super::InterruptFrame;


pub extern "x86-interrupt" fn breakpoint_gate(stack_frame: &mut InterruptFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:?}", stack_frame);
}


pub extern "x86-interrupt" fn double_fault_gate(stack_frame: &mut InterruptFrame, error_code: u64) {
    panic!("Double fault! {}\n{:?}", error_code, stack_frame);
}

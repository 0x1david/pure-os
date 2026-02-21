use crate::{gdt, println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    /// InterruptDescriptorTable is used by x86 architecture to handle exceptions and HW, SW interrupts
    /// When CPU receives an interrupt signal or encounters an exception, it uses the `interrupt
    /// vector` as an index into the IDT
    /// IDT can hold a maximum of 256 entries. Vectors 0-31 are rserved by Intel/AMD for specific processor
    /// excpetions (e.g. 14 -> Page Fault)
    ///
    /// Size of each entry depends on the CPU's operating mode:
    ///     Real Mode (IVT): 4 bytes (32 bits) per entry. Total table size = 1,024 bytes.
    ///     32-bit Protected Mode: 8 bytes (64 bits) per entry. Total table size = 2,048 bytes.
    ///     64-bit Long Mode: 16 bytes (128 bits) per entry. Total table size = 4,096 bytes.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

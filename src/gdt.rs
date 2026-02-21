use lazy_static::lazy_static;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    /// Historically (in 32-bit) the TSS was used for hardware-level context-switching - the CPU
    /// would automatically save and load all general-purpose registers to and from the TSS when
    /// switching between processes.
    ///
    /// Nowadays (in Long Mode) this purpose was deprecated and the TSS is strictly used for
    /// managing safe stack pointers (rsps) during privilege transitions and severe hardware exceptions.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: u64 = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE as usize] = [0; STACK_SIZE as usize];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);

            // x86 stacks grow downwards (from higher memory addresses to lower memory addresses).
            // We must provide the highest address of the allocation.
            stack_start + STACK_SIZE
        };
        tss
    };
}

lazy_static! {
    /// The GDT is a hardware-defined data structure used by x86 processors to manage memory
    /// segmentation and enforce execution privilege levels (protection rings)
    /// Acts as a lookup table containing an array of 8-byte entries called Segment Descriptors.
    /// When a segment register (cs, ds, ss, etc.) is loaded with a selector, the CPU indexes into
    /// the GT to determine the base linear adress, size limit, and access rights of that memory
    /// segment.
    ///
    /// Each entry contains:
    ///     32-bit Base Address (Pointer to start of the segment)
    ///     20-bit Segment Limit (The size)
    ///     Access Flags (ring level, permissions, segment type)
    ///
    /// In modern 64-bit mode (Long Mode), the base address and segment limit are ignored
    /// for primary segments (cs, ds, es, ss). The GDT is used primarily for access flags
    /// and privilege transitions, while memory protection is implemented entirely via paging.
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

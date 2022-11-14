use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

const STACK_DEFAULT_SIZE: usize = 4096 * 6; // TODO: fixed size stack

struct Gdt {
    gdt: GlobalDescriptorTable,
    cs: SegmentSelector,
    tss: SegmentSelector,
}

lazy_static! {
    static ref GDT: Gdt = {
        let mut gdt = GlobalDescriptorTable::new();
        let cs = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));

        Gdt { gdt, cs, tss }
    };
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[0] = {
            static mut STACK: [u8; STACK_DEFAULT_SIZE] = [0; STACK_DEFAULT_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_DEFAULT_SIZE;
            stack_end
        };
        tss
    };
}

pub fn init() {
    GDT.gdt.load();

    unsafe {
        CS::set_reg(GDT.cs);
        load_tss(GDT.tss);
    }
}

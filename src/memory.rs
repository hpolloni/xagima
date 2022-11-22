use bootloader::{
    bootinfo::{MemoryMap, MemoryRegionType},
    BootInfo,
};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::heap;

pub fn init(boot_info: &'static BootInfo) -> Result<(), MemoryError> {
    unsafe { unsafe_init(boot_info) }
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryError {
    PageMappingError,
    FrameAllocationError,
}

unsafe fn unsafe_init(boot_info: &'static BootInfo) -> Result<(), MemoryError> {
    let phys_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let (current_frame, _) = Cr3::read();
    let phys_addr = current_frame.start_address();
    let virt_addr = phys_offset + phys_addr.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    let level_4_table = &mut *page_table_ptr;

    let mut mapper = OffsetPageTable::new(level_4_table, phys_offset);

    let mut frame_allocator = DefaultFrameAllocator::new(&boot_info.memory_map);

    // TODO: hack for e1000 driver
    // The memory manager module should be decoupled from the mapper/frame allocation.
    // The interface should simply have an init and a map/map_to method.
    let page = Page::containing_address(VirtAddr::new(0xfebc0000));
    let frame = PhysFrame::containing_address(PhysAddr::new(0xfebc0000));
    mapper
        .map_to(
            page,
            frame,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            &mut frame_allocator,
        )?
        .flush();

    heap::init(&mut mapper, &mut frame_allocator)?;

    Ok(())
}

struct DefaultFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl DefaultFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let usable_regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for DefaultFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

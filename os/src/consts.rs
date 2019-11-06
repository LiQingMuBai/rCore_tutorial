pub const PHYSICAL_MEMORY_BEGIN: usize = 0x80000000;
pub const PHYSICAL_MEMORY_END: usize = 0x88000000;
pub const PHYSICAL_MEMORY_OFFSET: usize = 0xffffffff00000000;

pub const KERNEL_BEGIN_PADDR: usize = 0x80200000;
pub const KERNEL_BEGIN_VADDR: usize = 0xffffffffc0200000;

pub const MAX_PHYSICAL_MEMORY: usize = 0x8000000;
pub const MAX_PHYSICAL_PAGES: usize = MAX_PHYSICAL_MEMORY >> 12;

pub const PAGE_BYTES: usize = 4096;

pub const KERNEL_HEAP_SIZE: usize = 0x800000;


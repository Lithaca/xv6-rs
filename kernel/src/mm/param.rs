pub const KERNEL_HEAP_SIZE: usize = 0x30_0000; // kernel heap allocator size
pub const PAGE_SIZE: usize = 4096; // bytes per page
pub const PAGE_BITS: usize = 12; // bits of offset within a page

// physical address & virtual address
pub const PA_WIDTH_SV39: usize = 56;
pub const VA_WIDTH_SV39: usize = 39;
pub const MAX_PHYS_ADDR: usize = (1 << PA_WIDTH_SV39) - 1;
pub const MAX_VIRT_ADDR: usize = (1 << VA_WIDTH_SV39) - 1;
pub const MAX_PHYS_SIZE: usize = 1 << (PA_WIDTH_SV39 - 1);
pub const MAX_VIRT_SIZE: usize = 1 << (VA_WIDTH_SV39 - 1);

// physical page number & virtual page number
pub const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_BITS;
pub const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_BITS;
pub const MAX_PPN: usize = (1 << PPN_WIDTH_SV39) - 1;
pub const MAX_VPN: usize = (1 << VPN_WIDTH_SV39) - 1;

#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]

extern crate alloc;

#[macro_use]
mod io;

mod init;
mod lang_items;
mod sbi;
mod interrupt;
mod context;
mod timer;
mod consts;
mod memory;
mod process;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do nothing but panic!");
}

#[cfg(feature = "allocator")]
mod private {
    /****************************************************************/
    //                            Uses                              //
    /****************************************************************/

    use x86_64::{
        structures::paging::{
            mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
        },
        VirtAddr,
    };
    use alloc::alloc::{Layout, GlobalAlloc};
    use core::{ptr, mem};

    /****************************************************************/
    //                         Constants                            //
    /****************************************************************/

    pub const HEAP_START: usize = 0x_4444_4444_0000;
    pub const HEAP_SIZE: usize = 100 * 1024;

    const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

    /****************************************************************/
    //                            Types                             //
    /****************************************************************/

    pub struct Locked<A> {
        inner: spin::Mutex<A>,
    }

    impl<A> Locked<A> {
        pub const fn new(inner: A) -> Self {
            Locked {
                inner: spin::Mutex::new(inner),
            }
        }

        pub fn lock(&self) -> spin::MutexGuard<A> {
            self.inner.lock()
        }
    }

    struct Node {
        next: Option <&'static mut Node>
    }

    pub struct FixedSizeBlockAllocator {
        list: [Option <&'static mut Node>; BLOCK_SIZES.len()],
        fallback: linked_list_allocator::Heap
    }

    impl FixedSizeBlockAllocator {
        pub const fn new() -> Self {
            const EMPTY: Option <&'static mut Node> = None;

            FixedSizeBlockAllocator {
                list: [EMPTY; BLOCK_SIZES.len()],
                fallback: linked_list_allocator::Heap::empty()
            }
        }

        pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
            self.fallback.init(heap_start, heap_end);
        }

        fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
            match self.fallback.allocate_first_fit(layout) {
                Ok(ptr) => ptr.as_ptr(),
                Err(_) => ptr::null_mut()
            }
        }
    }

    unsafe impl GlobalAlloc for Locked <FixedSizeBlockAllocator> {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let mut allocator = self.lock();
            match list_index(&layout) {
                Some(index) => {
                    match allocator.list[index].take() {
                        Some(node) => {
                            allocator.list[index] = node.next.take();
                            node as *mut Node as *mut u8
                        }
                        None => {
                            let block_size = BLOCK_SIZES[index];
                            let block_align = block_size;
                            let layout = Layout::from_size_align(block_size, block_align).unwrap();
                            allocator.fallback_alloc(layout)
                        }
                    }
                }
                None => allocator.fallback_alloc(layout)
            }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            let mut allocator = self.lock();
            match list_index(&layout) {
                Some(index) => {
                    let new_node = Node {
                        next: allocator.list[index].take()
                    };
                    assert!(mem::size_of::<Node>() <= BLOCK_SIZES[index]);
                    assert!(mem::align_of::<Node>() <= BLOCK_SIZES[index]);
                    let new_node_ptr  = ptr as *mut Node;
                    new_node_ptr.write(new_node);
                    allocator.list[index] = Some(&mut *new_node_ptr);
                },
                None => {
                    let ptr = ptr::NonNull::new(ptr).unwrap();
                    allocator.fallback.deallocate(ptr, layout);
                }
            }
        }
    }

    /****************************************************************/
    //                           Statics                            //
    /****************************************************************/

    #[global_allocator]
    static ALLOCATOR: Locked <FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

    /****************************************************************/
    //                     Other functions                          //
    /****************************************************************/

    fn list_index(layout: &Layout) -> Option <usize> {
        let required = layout.size().max(layout.align());
        BLOCK_SIZES.iter().position(|&s| s >= required)
    }

    pub fn init_heap(mapper: &mut impl Mapper <Size4KiB>, frame_allocator: &mut impl FrameAllocator <Size4KiB>) -> Result <(), MapToError <Size4KiB>> {
        let range = {
            let start = VirtAddr::new(HEAP_START as u64);
            let end = start + HEAP_SIZE - 1u64;
            Page::range_inclusive(Page::containing_address(start), Page::containing_address(end))
        };

        for page in range {
            let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
            unsafe { mapper.map_to(page, frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, frame_allocator)?.flush() };
        }

        unsafe {
            ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
        }

        Ok(())
    }

    #[alloc_error_handler]
    fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
        panic!("allocation error: {:?}", layout)
    }

    pub mod frame {
        /****************************************************************/
        //                            Uses                              //
        /****************************************************************/

        use x86_64::{
            VirtAddr, PhysAddr,
            structures::paging::{
                PageTable, OffsetPageTable, PhysFrame, Size4KiB, FrameAllocator
            },
            registers::control::Cr3
        };
        use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

        /****************************************************************/
        //                            Types                             //
        /****************************************************************/

        pub struct BootInfoFrameAllocator {
            mmap: &'static MemoryMap,
            next: usize
        }

        impl BootInfoFrameAllocator {
            pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
                BootInfoFrameAllocator {
                    mmap: memory_map,
                    next: 0
                }
            }

            fn usable_frames(&self) -> impl Iterator <Item = PhysFrame> {
                self.mmap.iter().filter(|r| r.region_type == MemoryRegionType::Usable).map(|r| r.range.start_addr()..r.range.end_addr()).flat_map(|r| r.step_by(4096)).map(|address| PhysFrame::containing_address(PhysAddr::new(address)))
            }
        }

        unsafe impl FrameAllocator <Size4KiB> for BootInfoFrameAllocator {
            fn allocate_frame(&mut self) -> Option <PhysFrame> {
                let frame = self.usable_frames().nth(self.next);
                self.next += 1;
                frame
            }
        }

        /****************************************************************/
        //                     Other functions                          //
        /****************************************************************/

        pub unsafe fn init(offset: VirtAddr) -> OffsetPageTable <'static> {
            OffsetPageTable::new(get_active_p4(offset), offset)
        }

        unsafe fn get_active_p4(phys_offset: VirtAddr) -> &'static mut PageTable {
            let (p4, _) = Cr3::read();
            &mut *((phys_offset + p4.start_address().as_u64()).as_mut_ptr())
        }

    }
}

#[cfg(feature = "allocator")]
pub use private::*;

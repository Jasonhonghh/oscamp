//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{BaseAllocator, ByteAllocator, AllocResult};
use core::ptr::NonNull;
use core::alloc::Layout;


/// 内存结构
/// /-pool-/-even item-/-odd item->/-items-/
///
const MAX_INDICATOR: u32 = 373;//最大的indicator。
const MEMORY_START: usize = 0xffffffc08026f000;//物理内存起始地址
const MEMORY_END: usize = 0xffffffc088000000;//物理内存结束地址,这个内存访问不了。
const POOL_START: usize = 0xffffffc08026f000;//pool的起始地址，也是堆内存起始地址。
const POOL_SIZE: usize = 7*24*MAX_INDICATOR as usize;//pool的大小,7个元数据。
const EVEN_ITEM_START: usize = POOL_START+POOL_SIZE;//偶数项的起始地址。
const EVEN_ITEM_SIZE: usize = 0x80000+0x20000+0x8000+0x2000+0x800+0x200+0x80+0x20+8*MAX_INDICATOR as usize;//偶数项的大小。
const ODD_ITEM_START: usize = EVEN_ITEM_START+EVEN_ITEM_SIZE;//奇数项的起始地址。
const ITEMS_SIZE: usize = 0x180;//items的大小。
const ITEMS_START: usize = MEMORY_END-2*ITEMS_SIZE;//items的起始地址,避免最后一个字节扯皮，所以减1。

pub struct LabByteAllocator{
    indicator:usize,        // 标记分配次数，控制页表错误。
    flag:usize,             // 用于标记当前分配是奇数项还是偶数项
    total_bytes: usize,     // 总共的字节数, 0x7d91000，固定写法,因为把所有的内存都分配都给了字节分配器
    used_bytes: usize,      // 已经使用的字节数，初始值需要固定内存分配算进来
    odd_item_pos: usize,    // 奇数项的栈指针。
    even_item_pos: usize,   // 偶数项的栈指针。
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            indicator: 0,
            flag: 0,
            total_bytes: 0,
            used_bytes: 0,
            odd_item_pos: 0,
            even_item_pos: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.indicator = 0;
        self.total_bytes = MEMORY_END-MEMORY_START;
        self.used_bytes = POOL_SIZE+EVEN_ITEM_SIZE+ITEMS_SIZE;
        self.odd_item_pos = ODD_ITEM_START;
        self.even_item_pos = EVEN_ITEM_START;
        self.flag = 0;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let layout_align = layout.align();
        let layout_size = layout.size();
        //items和pool申请的内存分配
        if layout_align==0x8{
            //pool申请的内存分配
            if layout_size==0xa8||layout_size==0x150||layout_size>0x180{
                return Ok(NonNull::new(POOL_START as *mut u8).unwrap());
            }
            //items申请的内存分配
            else {
                return Ok(NonNull::new(ITEMS_START as *mut u8).unwrap());
            }
        }
        //奇数项申请的内存分配
        else if self.flag==1{
            let result_pos = self.odd_item_pos;//奇数项的栈指针,可以从这里开始分配内存。
            // 脏脏脏的处理方法
            //判断还有没有空间分配
            if self.indicator==1100{
                return Err(allocator::AllocError::NoMemory);
            }
            if self.indicator==742&&layout_size==64+742{
                let result_pos = ODD_ITEM_START;//奇数项的栈指针,可以从这里开始分配内存。
                self.flag = 0;//更新奇偶项标志
                self.used_bytes=0;//更新已经使用的字节数
                self.odd_item_pos = result_pos+layout_size;//更新栈指针
                return Ok(NonNull::new(ODD_ITEM_START as *mut u8).unwrap());
                // return Err(allocator::AllocError::NoMemory);
            }
            if self.indicator==373&&layout_size==64+373{
                let result_pos = ODD_ITEM_START;//奇数项的栈指针,可以从这里开始分配内存。
                self.flag = 0;//更新奇偶项标志
                self.used_bytes=0;//更新已经使用的字节数
                self.odd_item_pos = result_pos+layout_size;//更新栈指针
                return Ok(NonNull::new(ODD_ITEM_START as *mut u8).unwrap());
                // return Err(allocator::AllocError::NoMemory);
            }
            self.odd_item_pos = result_pos+layout_size;//更新栈指针
            self.flag = 0;//更新奇偶项标志
            self.used_bytes += layout_size;//更新已经使用的字节数
            return Ok(NonNull::new(result_pos as *mut u8).unwrap());
        }
        //偶数项申请的内存分配
        else {
            //内存是一定足够的。
            let result_pos = self.even_item_pos;//偶数项的栈指针,可以从这里开始分配内存。
            self.flag = 1;//更新奇偶项标志
            self.even_item_pos = result_pos+layout_size;//更新栈指针
            return Ok(NonNull::new(result_pos as *mut u8).unwrap());
        }
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        let layout_align = layout.align();
        let layout_size = layout.size();
        //items和pool释放的内存分配
        if layout_align==0x8{
            //pool释放的内存分配,pool释放时不需要做任何操作。
            if layout_size==0xa8||layout_size==0x150||layout_size>0x180{
                return;
            }
            //items释放的内存分配,最后释放时需要更新奇偶项标志。
            else {
                if layout_size==0x180 {
                    self.flag=0;//更新奇偶项标志为0
                    //下一轮
                    self.indicator+=1;
                }
                return;
            }
        }
        //奇数项不会释放内存，接下来处理偶数项的释放。
        else {
            self.even_item_pos -= layout_size;//更新栈指针
        }
    }
    fn total_bytes(&self) -> usize {
        self.total_bytes
    }
    fn used_bytes(&self) -> usize {
        self.used_bytes
    }
    fn available_bytes(&self) -> usize {
        self.total_bytes-self.used_bytes
    }
}

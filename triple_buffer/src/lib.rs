
use atomic_ref::AtomicRef;
use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
};

pub struct Trio {
    dirty: AtomicBool,
    buffer: [UnsafeCell<MaybeUninit<i32>>; 3],
}

impl Trio {
    pub fn new() -> Self {
        Self {
            dirty: AtomicBool::new(false),
            buffer: std::array::from_fn(|_| UnsafeCell::new(MaybeUninit::uninit())),
        }
    }

    pub fn read(&self) -> &i32 {
        let is_dirty = self.dirty.load(Ordering::Relaxed);
        let mut front_buffer = &self.buffer[0];
        if is_dirty {
            let middle_buffer = AtomicRef::new(Some(&self.buffer[1]));
            front_buffer = middle_buffer
            .swap(Some(front_buffer), Ordering::AcqRel)
            .unwrap();
        self.dirty.store(false, Ordering::Relaxed);
    }
    return unsafe {&(*front_buffer.get()).assume_init()};
}

    pub fn write(&self) -> &i32 {
        let back_buffer = &self.buffer[3];
        unsafe {&(*back_buffer.get()).assume_init()}
    }

    pub fn commit(&self) {
        let mut back_buffer = &self.buffer[3];
        let middle_buffer = AtomicRef::new(Some(&self.buffer[1]));
        back_buffer = middle_buffer
            .swap(Some(back_buffer), Ordering::AcqRel)
            .unwrap();
        self.dirty.store(true, Ordering::Relaxed);
    }
}

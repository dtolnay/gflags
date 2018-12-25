use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct StaticAtomicPtr<T> {
    ptr: AtomicPtr<T>,
}

impl<T> StaticAtomicPtr<T> {
    pub const fn new(value: &'static T) -> Self {
        StaticAtomicPtr {
            ptr: AtomicPtr::new(value as *const T as *mut T),
        }
    }

    pub const fn null() -> Self {
        StaticAtomicPtr {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn load(&self) -> &'static T {
        let ptr = self.ptr.load(Ordering::SeqCst);
        let reference = unsafe { ptr.as_ref() };
        match reference {
            Some(t) => t,
            None => panic!("flag is not present"),
        }
    }

    pub fn store(&self, value: &'static T) {
        let ptr = value as *const T as *mut T;
        self.ptr.store(ptr, Ordering::SeqCst);
    }
}

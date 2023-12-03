use std::{ptr::NonNull, cell::Cell};

struct BoxMutBox<T: Sized> {
    value: T,
    counter: Cell<usize>
}

pub struct BoxMut<T: Sized>(NonNull<BoxMutBox<T>>);

impl<T> BoxMut<T> {
    pub fn new(x: T) -> Self {
        Self (unsafe { NonNull::new_unchecked(Box::leak(Box::new(BoxMutBox {
            value: x,
            counter: Cell::new(1)
        })) as *mut BoxMutBox<T>) })
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut self.0.as_mut().value }
    }
}

impl<T> Clone for BoxMut<T> {
    fn clone(&self) -> Self {
        let boxed = unsafe { self.0.as_ref() };
        boxed.counter.set(boxed.counter.get() + 1);
        Self(self.0.clone())
    }
}

impl<T: Sized> Drop for BoxMut<T> {
    fn drop(&mut self) {
        let boxed = unsafe { self.0.as_ref() };
        if boxed.counter.get() == 1 {
            let _ = unsafe { Box::from_raw(self.0.as_ptr()) };
        } else {
            boxed.counter.set(boxed.counter.get() - 1);
        }
    }
}

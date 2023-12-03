use std::ptr::NonNull;

pub struct BoxMut<T: ?Sized>(NonNull<T>);

impl<T> BoxMut<T> {
    pub fn new(x: T) -> Self {
        Self (unsafe { NonNull::new_unchecked(Box::leak(Box::new(x)) as *mut T) })
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T> Clone for BoxMut<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for BoxMut<T> {}

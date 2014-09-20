extern crate libc;
use std::mem::size_of;
use std::ptr::write;
use libc::types::common::c95::c_void;

/// A "dumb" pointer that removes all safety garuntees.
pub struct U<T> {
    inner: *mut T
}

impl<T> U<T> {
    pub fn new(value: T) -> U<T> {
        unsafe {
            let ptr = libc::malloc(size_of::<T>() as u64) as *mut T;
            write(ptr, value);
            U { inner: ptr }
        }
    }

    pub fn free(&self) {
        unsafe {
            libc::free(self.inner as *mut c_void)
        }
    }
}

impl<T> Clone for U<T> {
    fn clone(&self) -> U<T> {
        U { inner: self.inner }
    }
}

impl<T> Deref<T> for U<T> {
    fn deref(&self) -> &T {
        unsafe { &*self.inner }
    }
}

impl<T> DerefMut<T> for U<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner }
    }
}


#[test]
fn test_integer() {
    let mut v = U::new(5i);
    assert!(*v == 5);
    *v = 10;
    assert!(*v == 10);

    let mut k = v.clone();
    *k = 30;

    assert!(*v == 30);
    assert!(*k == 30);
    k.free();
}

#[test]
fn test_struct() {
    struct Foo {
        x: uint,
        y: f32
    }
    impl Foo {
        fn sum(&self) -> f32 {
            self.y + self.x as f32
        }
        fn zero(&mut self) {
            self.x = 0;
            self.y = 0.0;
        }
    }

    let mut v = U::new(Foo{x: 50, y: 3.14});
    assert!(v.x == 50);
    assert!(v.y == 3.14);

    let mut k = v.clone();

    v.x = 30;
    v.y = -1.5;

    assert!(v.x == 30);
    assert!(v.y == -1.5);
    assert!(k.x == 30);
    assert!(k.y == -1.5);

    assert!(v.sum() == 28.5);
    assert!(k.sum() == 28.5);

    v.zero();
    assert!(v.sum() == 0.0);
    assert!(k.sum() == 0.0);
}

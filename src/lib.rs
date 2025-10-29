use std::{
    alloc::{Layout, alloc, dealloc},
    ops::{Deref, DerefMut},
    ptr::{NonNull, drop_in_place},
};

struct Inner<T> {
    value: T,
    ref_count: usize,
}

pub struct Shared<T> {
    ptr: NonNull<Inner<T>>,
}

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        let inner = Inner {
            value,
            ref_count: 1,
        };

        let layout = Layout::for_value(&inner);

        let raw_ptr = unsafe { alloc(layout) } as *mut Inner<T>;
        if raw_ptr.is_null() {
            panic!("[Shared] Allocation failed.")
        }

        unsafe {
            raw_ptr.write(inner);
        }

        Self {
            ptr: unsafe { NonNull::new_unchecked(raw_ptr) },
        }
    }
}

impl<T> Shared<T> {
    pub fn borrow(&self) -> &T {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.value
    }

    pub fn borrow_mut(&mut self) -> &mut T {
        let inner = unsafe { self.ptr.as_mut() };
        &mut inner.value
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl<T> DerefMut for Shared<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow_mut()
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        let raw_ptr = self.ptr.as_ptr();

        let inner = unsafe { &mut *raw_ptr };
        inner.ref_count += 1;

        Self { ptr: self.ptr }
    }
}

impl<T> Drop for Shared<T> {
    fn drop(&mut self) {
        let raw_ptr = self.ptr.as_ptr();

        let inner = unsafe { &mut *raw_ptr };
        inner.ref_count -= 1;

        if inner.ref_count == 0 {
            let layout = Layout::for_value(inner);
            unsafe {
                drop_in_place(raw_ptr);
                dealloc(raw_ptr as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Shared;

    #[test]
    fn read_and_write_numbers() {
        let mut x = Shared::new(10);
        assert_eq!(*x, 10);

        *x.borrow_mut() += 5;
        assert_eq!(*x, 15);
    }

    #[test]
    fn works_with_strings() {
        let mut s = Shared::new("Shared".to_string());
        assert_eq!(*s, "Shared");

        s.borrow_mut().push_str(" pointer!");
        assert_eq!(*s, "Shared pointer!");
    }

    #[test]
    fn works_with_structs() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut p = Shared::new(Point { x: 1, y: 2 });
        assert_eq!(*p, Point { x: 1, y: 2 });

        let p = p.borrow_mut();
        p.x = 10;
        p.y = 20;
        assert_eq!(*p, Point { x: 10, y: 20 });
    }

    #[test]
    fn works_with_vectors() {
        let mut v = Shared::new(vec![1, 2, 3]);
        assert_eq!(*v, vec![1, 2, 3]);

        v.borrow_mut().push(4);
        assert_eq!(*v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn multiple_clones() {
        let a = Shared::new(100);
        let mut b = a.clone();
        let c = a.clone();

        assert_eq!(*a, 100);
        assert_eq!(*b, 100);
        assert_eq!(*c, 100);

        *b.borrow_mut() += 50;
        assert_eq!(*a, 150);
        assert_eq!(*c, 150);
    }

    #[test]
    fn drop_frees_memory() {
        struct FreeMemory<'a> {
            flag: &'a mut bool,
        }

        impl<'a> Drop for FreeMemory<'a> {
            fn drop(&mut self) {
                *self.flag = true;
            }
        }

        let mut released = false;

        {
            let a = Shared::new(FreeMemory {
                flag: &mut released,
            });
            let _b = a.clone();
        }

        assert!(released);
    }
}

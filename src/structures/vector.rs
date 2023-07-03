use std::{alloc, ptr};
use std::alloc::{alloc, Layout, realloc};
use std::fmt::{Debug, Display, Formatter};
use std::ptr::NonNull;

pub struct Vec {
    ptr: NonNull<usize>,
    cap: usize,
    len: usize,
}

impl Vec {
    pub fn new() -> Self {
        Vec { len: 0, cap: 0, ptr: NonNull::dangling() }
    }

    pub fn len(&self) -> usize {
        return self.len.clone();
    }

    pub fn push(&mut self, el: usize) {
        if self.len == self.cap { self.grow(); }
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), el);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    fn grow(&mut self) {
        let new = if self.cap == 0 {
            let new_cap = 1;
            let new_layout = Layout::array::<usize>(new_cap).unwrap();
            (new_cap, new_layout, unsafe { alloc(new_layout) })
        } else {
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            let old_layout = Layout::array::<usize>(self.cap).unwrap();

            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<usize>(new_cap).unwrap();
            (new_cap, new_layout, unsafe { realloc(old_ptr, old_layout, new_layout.size()) })
        };

        let (new_cap, new_layout, new_ptr) = new;
        self.ptr = match NonNull::new(new_ptr as *mut usize) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl Debug for Vec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        unsafe {
            for i in 0..self.len {
                let v = ptr::read(self.ptr.as_ptr().add(i));
                if i != 0 { write!(f, ", ")? }
                write!(f, "{}: {}", i, v)?;
            }
            for i in self.len..self.cap {
                if i != 0 { write!(f, ", ")? }
                write!(f, "{}: {}", i, 0)?;
            }
        }
        write!(f, "]")
    }
}

impl Display for Vec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        unsafe {
            for i in 0..self.len {
                let v = ptr::read(self.ptr.as_ptr().add(i));
                if i != 0 { write!(f, ", ")? }
                write!(f, "{}", v)?;
            }
        }
        write!(f, "]")
    }
}

impl Drop for Vec {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() { }
            let layout = Layout::array::<usize>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_vector() {
        let vec = Vec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.to_string(), "[]");
        assert_eq!(format!("{:?}", vec), "[]");
    }

    #[test]
    fn should_push_to_vector() {
        let mut vec = Vec::new();
        vec.push(10);
        vec.push(1);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.to_string(), "[10, 1]");
        assert_eq!(format!("{:?}", vec), "[0: 10, 1: 1]");
    }

    #[test]
    fn should_pop_from_vector() {
        let mut vec = Vec::new();
        vec.push(10);
        vec.push(1);
        vec.pop();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.to_string(), "[10]");
        assert_eq!(format!("{:?}", vec), "[0: 10, 1: 0]");
    }
}

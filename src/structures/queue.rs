use crate::structures::vector;
use std::fmt::{Debug, Display, Formatter};

pub struct Queue {
    cur: vector::Vec,
    rev: vector::Vec,
}

impl Queue {
    pub fn new() -> Self {
        Queue { cur: vector::Vec::new(), rev: vector::Vec::new() }
    }

    pub fn len(&self) -> usize {
        return self.rev.len().clone() + self.cur.len().clone();
    }

    pub fn push(&mut self, el: usize) {
        for _ in 0..self.rev.len() {
            let tmp = self.rev.pop();
            self.cur.push(tmp.unwrap())
        }
        self.cur.push(el)
    }

    pub fn pop(&mut self) -> Option<usize> {
        for _ in 0..self.cur.len() {
            let tmp = self.cur.pop();
            self.rev.push(tmp.unwrap())
        }
        self.rev.pop()
    }
}

impl Debug for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.cur.len() != 0 {
            write!(f, "{:?}", self.cur)
        } else {
            write!(f, "{:?}", self.rev)
        }

    }
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.cur.len() != 0 {
            write!(f, "{}", self.cur)
        } else {
            write!(f, "{}", self.rev)
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_queue() {
        let vec = Queue::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.to_string(), "[]");
        assert_eq!(format!("{:?}", vec), "[]");
    }

    #[test]
    fn should_push_to_queue() {
        let mut vec = Queue::new();
        vec.push(10);
        vec.push(1);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.to_string(), "[10, 1]");
        assert_eq!(format!("{:?}", vec), "[0: 10, 1: 1]");
    }

    #[test]
    fn should_pop_from_queue() {
        let mut vec = Queue::new();
        vec.push(10);
        vec.push(1);
        vec.pop();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.to_string(), "[1]");
        assert_eq!(format!("{:?}", vec), "[0: 1, 1: 0]");
    }
}

use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;

// prev: Option<Weak<RefCell<Node>>>
// Option: 可能有前指针，也可能没有
// Weak: 弱引用，不会记录引用数
struct Node {
    value: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Link,
}

struct DoublyLinkedList {
    head: Link,
    tail: Link,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, value: i32) {
        let new = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }));
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new);
            }
            None => {
                self.head = Some(new.clone());
                self.tail = Some(new);
            }
        }
    }

    fn push_front(&mut self, value: i32) {
        let new = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new));
                new.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new);
            }
            None => {
                self.head = Some(new.clone());
                self.tail = Some(new);
            }
        }
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|old_tail| {
            if let Some(prev) = old_tail.borrow_mut().prev.take() {
                if let Some(prev_strong) = prev.upgrade() {
                    prev_strong.borrow_mut().next = None;
                    self.tail = Some(prev_strong);
                }
            } else {
                self.head.take();
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    fn iter(&self) -> Vec<i32> {
        let mut res = Vec::new();
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            res.push(node.borrow().value);
            cur = node.borrow().next.clone();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_front(0);
        list.push_back(2);
        println!("{:?}", list.iter()); // [0,1,2]
        println!("{:?}", list.pop_front()); // Some(0)
        println!("{:?}", list.pop_back()); // Some(2)
        println!("{:?}", list.iter()); // [1]
    }
}

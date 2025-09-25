use std::ptr;

struct Node {
    value: i32,
    prev: *mut Node,
    next: *mut Node,
}

struct DoublyLinkedList {
    head: *mut Node,
    tail: *mut Node,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    fn push_back(&mut self, value: i32) {
        unsafe {
            let new = Box::into_raw(Box::new(Node {
                value,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));
            if self.tail.is_null() {
                self.head = new;
                self.tail = new;
            } else {
                (*new).prev = self.tail;
                (*self.tail).next = new;
                self.tail = new;
            }
        }
    }

    fn push_front(&mut self, value: i32) {
        unsafe {
            let new = Box::into_raw(Box::new(Node {
                value,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));
            if self.head.is_null() {
                self.head = new;
                self.tail = new;
            } else {
                (*new).next = self.head;
                (*self.head).prev = new;
                self.head = new;
            }
        }
    }

    fn pop_back(&mut self) -> Option<i32> {
        unsafe {
            if self.tail.is_null() {
                return None;
            }
            let old_tail = self.tail;
            let prev = (*old_tail).prev;
            if prev.is_null() {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            } else {
                (*prev).next = ptr::null_mut();
                self.tail = prev;
            }
            Some(Box::from_raw(old_tail).value)
        }
    }

    fn pop_front(&mut self) -> Option<i32> {
        unsafe {
            if self.head.is_null() {
                return None;
            }
            let old_head = self.head;
            let next = (*old_head).next;
            if next.is_null() {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            } else {
                (*next).prev = ptr::null_mut();
                self.head = next;
            }
            Some(Box::from_raw(old_head).value)
        }
    }

    fn iter(&self) -> Vec<i32> {
        let mut res = Vec::new();
        unsafe {
            let mut cur = self.head;
            while !cur.is_null() {
                res.push((*cur).value);
                cur = (*cur).next;
            }
        }
        res
    }
}

impl Drop for DoublyLinkedList {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list() {
        let mut list = DoublyLinkedList::new();
        list.push_back(2);
        list.push_front(1);
        list.push_back(3);
        println!("{:?}", list.iter()); // [1,2,3]
        println!("{:?}", list.pop_front()); // Some(1)
        println!("{:?}", list.pop_back()); // Some(3)
        println!("{:?}", list.iter()); // [2]
    }
}

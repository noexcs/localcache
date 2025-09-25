use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: self.head.clone(),
        }));

        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().prev = Some(new_node.clone());
            self.head = Some(new_node);
        } else {
            // 空链表
            self.tail = Some(new_node.clone());
            self.head = Some(new_node);
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: self.tail.clone(),
            next: None,
        }));

        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            // 空链表
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        // take: Takes the value out of the option, leaving a None in its place.
        // map: Applies a function to a value in an option.
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            old_head.borrow_mut().prev = None;
            old_head.borrow_mut().next = None;

            // 改进：检查 try_unwrap 的结果
            match Rc::try_unwrap(old_head) {
                Ok(ref_cell) => ref_cell.into_inner().value,
                Err(_) => panic!("Unexpected additional references to node"),
            }
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(prev) = old_tail.borrow_mut().prev.take() {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                // 链表空了
                self.head.take();
            }
            // 手动打断 old_tail 的前后指针，避免循环引用
            old_tail.borrow_mut().prev = None;
            old_tail.borrow_mut().next = None;

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // 不断从前面弹出，直到为空
        while self.pop_front().is_some() {
            // 这里什么也不用做，pop_front 已经正确处理 Rc 的释放
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_front(0);
        // 0 1 2

        println!("pop_front: {:?}", list.pop_front()); // Some(0)
        println!("pop_back: {:?}", list.pop_back()); // Some(2)
        println!("pop_back: {:?}", list.pop_back()); // Some(1)
        println!("pop_back: {:?}", list.pop_back()); // None
    }
}

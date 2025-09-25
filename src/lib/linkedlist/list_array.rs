struct Node {
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

struct DoublyLinkedList {
    nodes: Vec<Node>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, value: i32) {
        let idx = self.nodes.len();
        let new = Node {
            value,
            prev: self.tail,
            next: None,
        };
        self.nodes.push(new);
        if let Some(tail) = self.tail {
            self.nodes[tail].next = Some(idx);
        } else {
            self.head = Some(idx);
        }
        self.tail = Some(idx);
    }

    fn push_front(&mut self, value: i32) {
        let idx = self.nodes.len();
        let new = Node {
            value,
            prev: None,
            next: self.head,
        };
        self.nodes.push(new);
        if let Some(head) = self.head {
            self.nodes[head].prev = Some(idx);
        } else {
            self.tail = Some(idx);
        }
        self.head = Some(idx);
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.tail.map(|idx| {
            let value = self.nodes[idx].value;
            let prev = self.nodes[idx].prev;
            if let Some(p) = prev {
                self.nodes[p].next = None;
                self.tail = Some(p);
            } else {
                self.head = None;
                self.tail = None;
            }
            value
        })
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.head.map(|idx| {
            let value = self.nodes[idx].value;
            let next = self.nodes[idx].next;
            if let Some(n) = next {
                self.nodes[n].prev = None;
                self.head = Some(n);
            } else {
                self.head = None;
                self.tail = None;
            }
            value
        })
    }

    fn iter(&self) -> Vec<i32> {
        let mut res = Vec::new();
        let mut cur = self.head;
        while let Some(idx) = cur {
            res.push(self.nodes[idx].value);
            cur = self.nodes[idx].next;
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
        list.push_back(20);
        list.push_front(10);
        list.push_back(30);
        println!("{:?}", list.iter()); // [10,20,30]
        println!("{:?}", list.pop_front()); // Some(10)
        println!("{:?}", list.pop_back()); // Some(30)
        println!("{:?}", list.iter()); // [20]
    }
}

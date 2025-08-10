use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

struct LinkedNode<T> {
    item: Option<T>, // Used to facilitate pops
    next: Option<usize>,
}

struct LinkedVector<T> {
    data: Vec<LinkedNode<T>>,
    head: Option<usize>,
    tail: Option<usize>,
    freelist: Vec<usize>,
}

impl<T> LinkedVector<T> {
    fn new() -> LinkedVector<T> {
        LinkedVector {
            data: Vec::new(),
            head: None,
            tail: None,
            freelist: Vec::new(),
        }
    }

    fn alloc(&mut self, new_node: LinkedNode<T>) -> usize {
        match self.freelist.pop() {
            Some(idx) => {
                self.data[idx] = new_node;
                return idx;
            }
            None => {
                self.data.push(new_node);
                return self.data.len() - 1;
            }
        }
    }

    pub fn push_front(&mut self, item: T) {
        let new_node = LinkedNode {
            item: Some(item),
            next: self.head,
        };
        let nidx = self.alloc(new_node);

        self.head = Some(nidx);
        if self.tail == None {
            // first element
            self.tail = Some(nidx);
        }
    }

    pub fn head(&self) -> Option<&LinkedNode<T>> {
        match self.head {
            Some(idx) => Some(&self.data[idx]),
            None => None,
        }
    }

    pub fn tail(&self) -> Option<&LinkedNode<T>> {
        match self.tail {
            Some(idx) => Some(&self.data[idx]),
            None => None,
        }
    }

    pub fn tail_mut(&mut self) -> Option<&mut LinkedNode<T>> {
        match self.tail {
            Some(idx) => Some(&mut self.data[idx]),
            None => None,
        }
    }

    pub fn push_back(&mut self, item: T) {
        let new_node = LinkedNode {
            item: Some(item),
            next: None,
        };
        let nidx = self.alloc(new_node);
        match self.tail_mut() {
            Some(tail_node) => {
                tail_node.next = Some(nidx);
                self.tail = Some(nidx);
            }
            None => {
                // New item is both head and tail
                self.head = Some(nidx);
                self.tail = Some(nidx);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head {
            None => None,
            Some(oidx) => {
                self.head = self.data[oidx].next;
                self.freelist.push(oidx);
                return Some(self.data[oidx].item.take().unwrap());
            }
        }
    }
    // pop_back not supported - for that, use a doubly implementation
    //   (or just use delete if you know the length)

    pub fn delete(&mut self, idx: usize) -> T {
        // Will panic if idx out of bounds
        if idx == 0 {
            return self.pop_front().unwrap();
        }

        self.freelist.push(self[idx - 1].next.unwrap());

        match self[idx].next {
            Some(next_idx) => {
                // idx + 1 exists, needs rerouting
                self[idx - 1].next = Some(next_idx);
            }
            None => {
                // idx + 1 doesn't exist, simply change second-to-last to none
                self[idx - 1].next = None;
            }
        }
        self.data[idx].item.take().unwrap()
    }
}

impl<T> Index<usize> for LinkedVector<T> {
    type Output = LinkedNode<T>;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current = self.head;
        for _ in 0..index {
            current = self.data[current.expect("Index not within bounds")].next;
        }
        &self.data[current.expect("Index not within bounds")]
    }
}

impl<T> IndexMut<usize> for LinkedVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut current = self.head;
        for _ in 0..index {
            current = self.data[current.expect("Index not within bounds")].next;
        }
        &mut self.data[current.expect("Index not within bounds")]
    }
}

impl<T: Debug> Debug for LinkedNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item.as_ref().unwrap())
    }
}

impl<T: Debug> Debug for LinkedVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut idx = self.head;
        while let Some(idx_num) = idx {
            idx = self.data[idx_num].next;
            match write!(f, "{:?} ", self.data[idx_num]) {
                Ok(_) => (),
                Err(err) => return Err(err),
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut my_vec = LinkedVector::<u64>::new();
        assert_eq!(format!("{my_vec:?}").trim(), "");

        my_vec.push_back(100u64);
        assert_eq!(format!("{my_vec:?}").trim(), "100");

        my_vec.push_back(200u64);
        assert_eq!(format!("{my_vec:?}").trim(), "100 200");

        my_vec.push_front(300u64);
        assert_eq!(format!("{my_vec:?}").trim(), "300 100 200");

        let a = my_vec.pop_front();
        assert_eq!(format!("{my_vec:?}").trim(), "100 200");
        assert_eq!(a, Some(300));

        let b = my_vec.delete(1);
        assert_eq!(format!("{my_vec:?}").trim(), "100");
        assert_eq!(b, 200);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        let mut my_vec = LinkedVector::<u64>::new();

        my_vec.push_back(100u64);
        my_vec.push_back(200u64);
        my_vec.push_front(300u64);

        my_vec.pop_front();
        my_vec.delete(1);

        my_vec.delete(1); // Should panic here
    }
}

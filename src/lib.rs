use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

pub struct LinkedNode<T> {
    pub item: Option<T>, // Option<> is used to facilitate pops (.take())
    next: Option<usize>,
}

pub struct LinkedVector<T> {
    data: Vec<LinkedNode<T>>,
    head: Option<usize>,
    tail: Option<usize>,
    freelist: Vec<usize>,
    length: usize,
}

impl<T> LinkedVector<T> {
    pub fn new() -> LinkedVector<T> {
        LinkedVector {
            data: Vec::new(),
            head: None,
            tail: None,
            freelist: Vec::new(),
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        // Number of linked nodes
        debug_assert_eq!(self.data.len(), self.length + self.freelist.len());
        self.length
    }

    pub fn true_len(&self) -> usize {
        // Length of the underlying vector (maximum length used during lifetime)
        debug_assert_eq!(self.data.len(), self.length + self.freelist.len());
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        // Capacity of the underlying vector
        self.data.capacity()
    }

    pub fn mem_used(&self) -> usize {
        // Gives an estimate of the total *heap* memory acitvely used, in bytes
        // Calculation formula:
        //    data: (size_of(T) + usize) * data.len()
        //    freelist: usize * freelist.len()
        (size_of::<T>() + size_of::<usize>()) * self.data.len()
            + size_of::<usize>() * self.freelist.len()
    }

    pub fn true_mem_used(&self) -> usize {
        // Gives an estimate of the total *heap* memory allocated, in bytes
        // Calculation formula:
        //    data: (size_of(T) + usize) * data.capacity()
        //    freelist: usize * freelist.capacity()
        (size_of::<T>() + size_of::<usize>()) * self.data.capacity()
            + size_of::<usize>() * self.freelist.capacity()
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

        self.length += 1;
    }

    pub fn head(&self) -> Option<&LinkedNode<T>> {
        self.head.map(|idx| &self.data[idx])
    }

    pub fn head_mut(&mut self) -> Option<&mut LinkedNode<T>> {
        self.head.map(|idx| &mut self.data[idx])
    }

    pub fn tail(&self) -> Option<&LinkedNode<T>> {
        self.tail.map(|idx| &self.data[idx])
    }

    pub fn tail_mut(&mut self) -> Option<&mut LinkedNode<T>> {
        self.tail.map(|idx| &mut self.data[idx])
    }

    pub fn push_back(&mut self, item: T) {
        let nidx = self.alloc(LinkedNode {
            item: Some(item),
            next: None,
        });

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

        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.length -= 1;
        self.head.map(|oidx| {
            self.head = self.data[oidx].next;
            self.freelist.push(oidx);
            self.data[oidx].item.take().unwrap()
        })
    }

    // pop_back not supported since it's not an efficient operation.
    //   To achieve that, ideally implement with a doubly linked list,
    //   or just use delete() with len().
    #[cfg(any())]
    pub fn pop_back(&mut self) -> () {}

    // Returns the physical index in data for an index.
    // Panics if out-of-bounds.
    fn physical_index_of(&self, index: usize) -> usize {
        let mut current = self.head.expect("Index out of bounds");
        for _ in 0..index {
            current = self.data[current].next.expect("Index out of bounds");
        }
        current
    }

    pub fn delete(&mut self, idx: usize) -> T {
        // Indexing will panic if idx out of bounds.
        // This debug assert is intended to panic earlier
        //   during debugs to improve clarity.
        debug_assert!(idx < self.length);

        if idx == 0 {
            return self.pop_front().unwrap();
        }

        let prev_phys = self.physical_index_of(idx - 1);
        let remove_phys = self.data[prev_phys].next.expect("Index out of bounds");

        self.data[prev_phys].next = self.data[remove_phys].next;
        self.freelist.push(remove_phys);

        self.length -= 1;

        self.data[idx].item.take().unwrap()
    }
}

impl<T> Index<usize> for LinkedVector<T> {
    type Output = LinkedNode<T>;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current = self.head;
        for _ in 0..index {
            current = self.data[current.expect("Index out of bounds")].next;
        }
        &self.data[current.expect("Index out of bounds")]
    }
}

impl<T> IndexMut<usize> for LinkedVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut current = self.head;
        for _ in 0..index {
            current = self.data[current.expect("Index out of bounds")].next;
        }
        &mut self.data[current.expect("Index out of bounds")]
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

        assert_eq!(my_vec.len(), 0);
        assert_eq!(my_vec.true_len(), 0);

        my_vec.push_back(100u64);
        assert_eq!(format!("{my_vec:?}").trim(), "100");

        my_vec.push_back(200u64);
        assert_eq!(format!("{my_vec:?}").trim(), "100 200");

        my_vec.push_front(300u64);
        assert_eq!(format!("{my_vec:?}").trim(), "300 100 200");

        assert_eq!(my_vec.len(), 3);
        assert_eq!(my_vec.true_len(), 3);

        let a = my_vec.pop_front();
        assert_eq!(format!("{my_vec:?}").trim(), "100 200");
        assert_eq!(a, Some(300));

        assert_eq!(my_vec.len(), 2);
        assert_eq!(my_vec.true_len(), 3);

        let b = my_vec.delete(1);
        assert_eq!(format!("{my_vec:?}").trim(), "100");
        assert_eq!(b, 200);

        assert_eq!(my_vec.len(), 1);
        assert_eq!(my_vec.true_len(), 3);
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

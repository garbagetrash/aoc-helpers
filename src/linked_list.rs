use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct LinkedListNode<T> {
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub value: T,
}

impl<T> LinkedListNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            head: None,
            tail: None,
            value,
        }
    }

    pub fn next(&self) -> Option<usize> {
        // Tail is what is to come
        self.tail
    }

    pub fn prev(&self) -> Option<usize> {
        // Head is what came before
        self.head
    }
}

#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    pub data: Vec<LinkedListNode<T>>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
    pub size: usize,
}

impl<T: Copy + Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push_tail(&mut self, value: T) -> usize {
        let mut new_node = LinkedListNode::new(value);
        new_node.head = self.tail;
        let new_id = self.data.len();
        if let Some(tail_id) = self.tail {
            self.data[tail_id].tail = Some(new_id);
            self.tail = Some(new_id);
        } else {
            self.head = Some(new_id);
            self.tail = Some(new_id);
        }

        self.data.push(new_node);
        self.size += 1;

        new_id
    }

    pub fn push_head(&mut self, value: T) -> usize {
        let mut new_node = LinkedListNode::new(value);
        new_node.tail = self.head;
        let new_id = self.data.len();
        if let Some(head_id) = self.head {
            self.data[head_id].head = Some(new_id);
            self.head = Some(new_id);
        } else {
            self.head = Some(new_id);
            self.tail = Some(new_id);
        }

        self.data.push(new_node);
        self.size += 1;

        new_id
    }

    /// Returns the new node ID.
    pub fn insert_after(&mut self, value: T, node_id: usize) -> usize {
        let mut new_node = LinkedListNode::new(value);
        let new_id = self.data.len();
        new_node.head = Some(node_id);
        new_node.tail = self.data[node_id].tail;
        self.data[node_id].tail = Some(new_id);
        if let Some(next_id) = new_node.tail {
            self.data[next_id].head = Some(new_id);
        } else {
            // If no tail to node_id, then it was the list tail
            self.tail = Some(new_id);
        }
        self.data.push(new_node);
        self.size += 1;
        new_id
    }

    /// NOTE: This removes from the list, but not the underlying data vector.
    pub fn pop_id(&mut self, node_id: usize) -> T {
        if node_id > self.data.len() {
            panic!("Tried to remove id not in list");
        }
        let head_id = self.data[node_id].head;
        let tail_id = self.data[node_id].tail;

        if let Some(hid) = head_id {
            self.data[hid].tail = tail_id;
        } else {
            // If no head to node_id, then it was the list head
            self.head = tail_id;
        }
        if let Some(tid) = tail_id {
            self.data[tid].head = head_id;
        } else {
            // If no tail to node_id, then it was the list tail
            self.tail = head_id;
        }
        self.size -= 1;
        self.data[node_id].value
    }

    pub fn print(&self) {
        if let Some(hid) = self.head {
            let mut id = hid;
            print!("{:?} ", self.data[id].value);
            if Some(id) == self.data[id].next() {
                println!("");
                return;
            }

            loop {
                if let Some(next_id) = self.data[id].next() {
                    id = next_id;
                    print!("{:?} ", self.data[id].value);
                } else {
                    break;
                }
                if id == self.tail.unwrap() {
                    break;
                }
            }
            println!("");
        }
    }
}

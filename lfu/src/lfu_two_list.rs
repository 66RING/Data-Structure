use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
    key: i32,
    value: i32,
    freq: i32,
}

impl Node {
    pub fn new(key: i32, value: i32) -> Node {
        Self {
            prev: None,
            next: None,
            key,
            value,
            freq: 1,
        }
    }
}

#[derive(Default, Debug)]
pub struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl List {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn pop_back(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(old_tail) = self.tail.take() {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    // 清空新tail
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None
                }
            }
            Some(old_tail)
        } else {
            None
        }
    }

    pub fn push_front(&mut self, node: Rc<RefCell<Node>>) {
        let new_head = node;
        match self.head.take() {
            Some(old_head) => {
                new_head.borrow_mut().next = Some(old_head.clone());
                old_head.borrow_mut().prev = Some(new_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head.clone());
            }
        }
    }

    pub fn erase(&mut self, node: Rc<RefCell<Node>>) {
        let mut node = node.borrow_mut();
        match (node.prev.take(), node.next.take()) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());
            }
            (Some(prev), None) => {
                // erase tail
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            }
            (None, Some(next)) => {
                // erase head
                next.borrow_mut().prev = None;
                self.head = Some(next);
            }
            (None, None) => {
                // erase single node
                self.head = None;
                self.tail = None;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

struct LFUCache {
    capacity: i32,
    // key, value map
    key_table: HashMap<i32, Rc<RefCell<Node>>>,
    freq_table: HashMap<i32, List>,
    min_freq: i32,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            key_table: HashMap::new(),
            freq_table: HashMap::new(),
            min_freq: 1,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        match self.key_table.get(&key) {
            Some(node) => {
                let v = node.borrow_mut().value;
                self.upgrade(node.clone());
                v
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        match self.key_table.get(&key) {
            Some(node) => {
                // 如果存在则更新节点值和lfu值
                node.borrow_mut().value = value;
                self.upgrade(node.clone());
            }
            None => {
                // 如果不存在, 则插入新节点
                let new_node = Rc::new(RefCell::new(Node::new(key, value)));
                if self.capacity == self.key_table.len() as i32 {
                    // 如果容量已满
                    // 淘汰min_freq的链尾, 删除对应的key_table
                    let enode = self.freq_table.get_mut(&self.min_freq).unwrap().pop_back();
                    self.key_table.remove(&enode.unwrap().borrow().key);
                    // 如果list空了要情况内存
                    if self.freq_table.get(&self.min_freq).unwrap().is_empty() {
                        self.freq_table.remove(&self.min_freq);
                    }
                }
                // 新节点插入key_table和freq_table对应链的链首
                //  注意freq list不存在时要先插入
                self.key_table.insert(key, new_node.clone());
                self.freq_table
                    .entry(1)
                    .or_insert(List::new())
                    .push_front(new_node);
                // 新节点的freq始终为1, 更新最小freq
                self.min_freq = 1;
            }
        }
    }

    fn upgrade(&mut self, node: Rc<RefCell<Node>>) {
        let freq = node.borrow().freq;
        let elist = self.freq_table.get_mut(&freq).unwrap();
        elist.erase(node.clone());
        // 如果erase后freq list为空, 且是min_freq, 说明没有再小的freq了, 更新最小freq
        if elist.is_empty() {
            self.freq_table.remove(&freq);
            if self.min_freq == freq {
                self.min_freq = freq + 1;
            }
        }
        // 插入新的freq list中
        //  注意freq list不存在时要先插入
        node.borrow_mut().freq += 1;
        self.freq_table
            .entry(freq + 1)
            .or_insert(List::new())
            .push_front(node.clone());
    }
}

#[cfg(test)]
mod test {
    use super::{List, LFUCache, Node};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn basics_lfu() {

        let mut l = LFUCache::new(3);
        l.put(2, 2);
        l.put(1, 1);
        assert_eq!(l.get(2), 2);
        assert_eq!(l.get(1), 1);
        assert_eq!(l.get(2), 2);
        l.put(3, 3);
        l.put(4, 4);
        assert_eq!(l.get(3), -1);
        assert_eq!(l.get(2), 2);
        assert_eq!(l.get(1), 1);
        assert_eq!(l.get(4), 4);

        let mut l = LFUCache::new(2);
        l.put(2, 1);
        l.put(2, 2);
        assert_eq!(l.get(2), 2);
        l.put(1, 1);
        l.put(4, 1);
        assert_eq!(l.get(2), 2);

        let mut l = LFUCache::new(2);
        l.put(1, 1);
        l.put(2, 2);
        assert_eq!(l.get(1), 1);
        l.put(3, 3);
        assert_eq!(l.get(2), -1);
        assert_eq!(l.get(3), 3);
        l.put(4, 4);
        assert_eq!(l.get(1), -1);
        assert_eq!(l.get(3), 3);
        assert_eq!(l.get(4), 4);

    }
}

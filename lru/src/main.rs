use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

struct LRUNode<K: Hash + Eq + PartialEq + Clone, V: Clone> {
    next: Option<Rc<RefCell<LRUNode<K, V>>>>,
    prev: Option<Rc<RefCell<LRUNode<K, V>>>>,
    key: K,
    value: V,
}

struct LRUCache<K: Hash + Eq + PartialEq + Clone, V: Clone> {
    head: Option<Rc<RefCell<LRUNode<K, V>>>>,
    map: HashMap<K, Rc<RefCell<LRUNode<K, V>>>>,
    capacity: i32,
    len: i32,
}


impl<K: Hash + Eq + PartialEq + Clone, V: Clone> LRUNode<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { next: None, prev: None, key, value }
    }
}

impl<K: Hash + Eq + PartialEq + Clone, V: Clone> LRUCache<K, V> {
    fn new(capacity: i32) -> Self {
        Self { head: None, map: HashMap::new(), capacity, len: 0 }
    }
    
    // 使用hashmap索引查找
    fn get(&mut self, key: K) -> Option<V> {
        // 如果存在则移到链首
        if let Some(node) = self.map.get(&key) {
            let v = node.borrow_mut().value.clone();
            self.move_to_head(node.clone());
            return Some(v);
        } else {
            return None;
        }
    }
    
    fn put(&mut self, key: K, value: V) {
        // 如果key已存在则覆盖, 并移动到链首
        if let Some(node) = self.map.get(&key) {
            node.borrow_mut().value = value;
            self.move_to_head(node.clone());
        } else {
            // 不存在则在链首插入
            if self.len == self.capacity {
                let t = self.head.as_ref().unwrap().borrow_mut().prev.as_ref().unwrap().clone();
                // 如果LRU缓存已满, 则删除链尾元素
                //  1. 删除链表节点和哈希表节点
                self.delete(t.clone());
                //  2. 链首插入新元素, 包括接节点和hashmap
                let node = Rc::new(RefCell::new(LRUNode::new(key.clone(), value.clone())));
                //      2.1 插入链首
                self.move_to_head(node.clone());
                //      2.2 插入hashmap
                self.map.insert(key, node.clone());
            } else {
                self.len += 1;
                // 如果LRU缓存未满
                //  1. 链首插入新元素, 包括接节点和hashmap
                let node = Rc::new(RefCell::new(LRUNode::new(key.clone(), value.clone())));
                //      1.1 插入链首
                self.move_to_head(node.clone());
                //      1.2 插入hashmap
                self.map.insert(key, node.clone());
            }
        }
    }

    fn delete(&mut self, node: Rc<RefCell<LRUNode<K, V>>>) {
        self.map.remove(&node.borrow().key);
        let prev = node.borrow_mut().prev.as_ref().unwrap().clone();
        let next = node.borrow_mut().next.as_ref().unwrap().clone();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev.clone());
    }

    fn move_to_head(&mut self, node: Rc<RefCell<LRUNode<K, V>>>) {
        if self.head.is_none() {
            self.head = Some(node.clone());
            node.borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = Some(node.clone());
        } else if  self.head.as_ref().unwrap().borrow().key == node.as_ref().borrow().key {
            return;
        } else {
            // 插入新节点时: prev, next为空
            if node.borrow_mut().prev.is_some() {
                let prev = node.borrow_mut().prev.as_ref().unwrap().clone();
                let next = node.borrow_mut().next.as_ref().unwrap().clone();
                // 如果是不是新节点, 其prev和next将非空
                // 断开连接
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());
            }

            // 移动到链首
            let h = self.head.as_ref().unwrap().clone();
            let t = self.head.as_ref().unwrap().borrow_mut().prev.as_ref().unwrap().clone();
            // 与旧head相连
            node.borrow_mut().next = Some(h.clone());
            // 与旧tail相连
            node.borrow_mut().prev = Some(t.clone());
            h.borrow_mut().prev = Some(node.clone());
            t.borrow_mut().next = Some(node.clone());
            self.head = Some(node);
        }
    }

    fn erase(&mut self) { }
    fn pop_back(&mut self) { }
    fn push_front(&mut self, node: Rc<RefCell<LRUNode<K, V>>>) { }
}


#[test]
fn test() {
    // ["LRUCache","put","put","get","put","get",// "put","get","get","get"]
    // [[2],[1,1],[2,2],[1],[3,3], // [2],[4,4],[1],[3],[4]]
    let mut obj = LRUCache::new(2);
    obj.put(1, 1);
    obj.put(2, 2);
    // 1 -> 2
    assert_eq!(Some(1), obj.get(1));
    // 3 -> 1
    obj.put(3, 3);
    // should be -1 since lru cap = 2
    assert_eq!(None, obj.get(2));
    // 4 -> 3
    obj.put(4, 4);
    assert_eq!(None, obj.get(1));
    assert_eq!(Some(3), obj.get(3));
    assert_eq!(Some(4), obj.get(4));

    // ["LRUCache","put","put","get",// "put","get","put","get","get","get"]
    // [[2],[1,0],[2,2],[1]// ,[3,3],[2],[4,4],[1],[3],[4]]
    let mut obj = LRUCache::new(2);
    // 1:0 
    obj.put(1, 0);
    // 2:2 -> 1:0
    obj.put(2, 2);
    // 1:0 -> 2:2
    assert_eq!(Some(0), obj.get(1));
    // 3:3 -> 1:0
    obj.put(3, 3);
    assert_eq!(None, obj.get(2));
    // 4:4 -> 3:3
    obj.put(4, 4);
    assert_eq!(None, obj.get(1));
    assert_eq!(Some(3), obj.get(3));
    assert_eq!(Some(4), obj.get(4));

    // ["LRUCache","put","get","put","get","get"]
    // [[1],[2,1],[2],[3,2],[2],[3]]
    let mut obj = LRUCache::new(1);
    obj.put(2, 1);
    assert_eq!(Some(1), obj.get(2));
    obj.put(3, 2);
    assert_eq!(None, obj.get(2));
    assert_eq!(Some(2), obj.get(3));

    // ["LRUCache","put","put","get","get","put","get","get","get"]
    // [[2],[2,1],[3,2],[3],[2],[4,3],[2],[3],[4]]
    // 头节点被使用, rehead的情况
    let mut obj = LRUCache::new(2);
    // 2:1
    obj.put(2, 1);
    assert_eq!(Some(1), obj.get(2));
    // 3:2 -> 2:1
    obj.put(3, 2);
    assert_eq!(Some(2), obj.get(3));
    // 2:1 -> 3:2
    assert_eq!(Some(1), obj.get(2));
    // 4:3 -> 2:1
    obj.put(4, 3);
    // 2:1 -> 4:3
    assert_eq!(Some(1), obj.get(2));
    assert_eq!(None, obj.get(3));
    // 4:3 -> 2:1
    assert_eq!(Some(3), obj.get(4));
}



fn main() {
}

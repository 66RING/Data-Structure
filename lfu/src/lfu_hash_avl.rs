use std::cmp::Ordering;
use std::collections::{HashMap, BTreeSet};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct LFUNode<K, V> {
    // 最后一次使用的时间
    time: i32,
    // 频率
    cnt: i32,
    key: K,
    value: V,
}

struct LFUCache {
    key_table: HashMap<i32, LFUNode<i32, i32>>,
    order_set: BTreeSet<LFUNode<i32, i32>>,
    capacity: i32,
    time: i32,
}

impl<K: Eq, V: Eq> Ord for LFUNode<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        // f = cnt/T, 如果cnt相同则淘汰最久未使用的
        match self.cnt.cmp(&other.cnt) {
            Ordering::Equal => self.time.cmp(&other.time),
            _o => _o,
        }
    }
}
impl<K: Eq, V: Eq> PartialOrd for LFUNode<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {

    fn new(capacity: i32) -> Self {
        Self { key_table: HashMap::new(), order_set: BTreeSet::new(), capacity, time: 0 }
    }
    
    // 获取元素, 更新lfu信息， 返回value, 不存在则返回-1
    fn get(&mut self, key: i32) -> i32 {
        if let Some(cache) = self.key_table.get_mut(&key) {
            // 更新
            self.order_set.remove(&cache);
            cache.cnt += 1;
            cache.time = self.time + 1;
            self.time += 1;
            self.order_set.insert(*cache);
            return cache.value;
        } else {
            return -1;
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return
        }
        if let Some(cache) = self.key_table.get_mut(&key) {
            // 如果存在, 则覆盖
            //  更新table
            self.order_set.remove(cache);
            //  更新"lfu值"
            cache.time = self.time + 1;
            cache.cnt += 1;
            cache.value = value;
            self.time += 1;
            self.order_set.insert(*cache);

        } else {
            // 如果不存在
            // 如果空间已满
            if self.capacity as usize == self.key_table.len() {
                // 在set和map中都删除"lfu值"最小的元素
                let &e = self.order_set.iter().next().unwrap();
                self.key_table.remove(&e.key);
                self.order_set.remove(&e);
            }
            // 插入新元素
            let node = LFUNode { time: self.time + 1, cnt: 1, key, value };
            self.time += 1;
            self.key_table.insert(key, node.clone());
            self.order_set.insert(node);
        }
    }
}


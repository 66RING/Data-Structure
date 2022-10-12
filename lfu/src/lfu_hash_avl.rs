use std::cmp::Ordering;
use std::collections::{HashMap, BTreeSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct LFUNode<K, V> {
    // 最后一次使用的时间
    time: i32,
    // 频率
    cnt: i32,
    key: K,
    value: V,
}

struct LFUCache<K, V> {
    key_table: HashMap<K, LFUNode<K, V>>,
    order_set: BTreeSet<LFUNode<K, V>>,
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


impl<K: Hash + Eq + Clone + Copy, V: Eq + Clone + Copy> LFUCache<K, V> {
    fn new(capacity: i32) -> Self {
        Self { key_table: HashMap::new(), order_set: BTreeSet::new(), capacity, time: 0 }
    }
    
    // 获取元素, 更新lfu信息， 返回value, 不存在则返回-1
    fn get(&mut self, key: K) -> Option<V> {
        if let Some(cache) = self.key_table.get_mut(&key) {
            // 更新
            self.order_set.remove(&cache);
            cache.cnt += 1;
            cache.time = self.time + 1;
            self.time += 1;
            self.order_set.insert(*cache);
            Some(cache.value)
        } else {
            None
        }
    }
    
    fn put(&mut self, key: K, value: V) {
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

#[cfg(test)]
mod test {
    use super::LFUCache;
    #[test]
    fn basics() {
        let mut l = LFUCache::new(2);
        l.put(3, 1);
        l.put(2, 1);
        l.put(2, 2);
        l.put(4, 4);
        // 4 3
        // 2
        assert_eq!(l.get(2), Some(2));

        let mut l = LFUCache::new(2);
        l.put(1, 1);
        l.put(2, 2);
        assert_eq!(l.get(1), Some(1));
        // 3 x2
        // 1
        l.put(3, 3);
        assert_eq!(l.get(2), None);
        // x2
        // 3 1
        assert_eq!(l.get(3), Some(3));
        // 4 x2
        // 3 x1
        l.put(4, 4);
        assert_eq!(l.get(1), None);
        assert_eq!(l.get(3), Some(3));
        assert_eq!(l.get(4), Some(4));
    }
}

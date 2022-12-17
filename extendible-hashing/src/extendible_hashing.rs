#![allow(dead_code)]
use std::collections::{HashMap};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::BitAnd;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

#[derive(Debug)]
struct Bucket<K, V> {
    table: HashMap<K, V>,
    bucket_cap: usize,
    local_depth: usize,
}

pub struct ExtendiableHash<K, V> {
    entries: Vec<Rc<RefCell<Bucket<K, V>>>>,
    global_depth: usize,
    bucket_cap: usize,
}

pub enum Mode {
    No,     // 什么都不做
    Merge,  // 自动合并
    Shrink, // 自动合并加压缩
}

impl<K, V> Bucket<K, V> 
    where K: Hash + Eq + Clone, V: std::fmt::Display + Clone + Copy + Ord
{
    pub fn new(bucket_cap: usize, local_depth: usize) -> Self {
        let mut table = HashMap::new();
        // 提前预分配
        table.reserve(bucket_cap);
        Self {
            table,
            bucket_cap,
            local_depth,
        }
    }

    /// 当容量满时返回false, 触发分裂
    pub fn insert(&mut self, key: K, value: V) -> bool {
        // 如果不存在才插入, 不做覆盖
        self.table.entry(key).or_insert(value);
        true
    }

    /// 直接覆盖key对应的value
    pub fn update(&mut self, key: K, value: V) {
        // 直接覆盖
        self.table.entry(key).and_modify(|v| *v = value);
    }

    /// 删除kv
    pub fn remove(&mut self, key: &K) {
        self.table.remove(key);
    }

    /// 查询kv
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.table.get_mut(key)
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.table.get(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.table.contains_key(&key)
    }

    pub fn is_full(&self) -> bool {
        self.table.len() >= self.bucket_cap
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// rust getter, setter命名规范: getter函数同变量名
    pub fn local_depth(&self) -> usize {
        self.local_depth
    }

    pub fn table_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.table
    }
    /// 增加depth值, 返回新depth
    pub fn depth_up(&mut self) -> usize {
        self.local_depth += 1;
        self.local_depth
    }
    /// 减小depth值, 返回新depth
    pub fn depth_down(&mut self) -> usize {
        self.local_depth -= 1;
        self.local_depth
    }

    /// 获取分裂后新桶id
    /// 因为可扩展哈希, 所以只有local_depth范围下的最高位不同
    pub fn pair_index(&self, bucket_id: usize) -> usize {
        (1 << (self.local_depth-1)) ^ bucket_id
    }

    /// 显示桶中的数据
    pub fn display(&self) {
        let mut ve = vec![];
        for (_, v) in &self.table {
            ve.push(v);
        }
        ve.sort();
        for v in ve {
            print!("{} ", v);
        }
        println!();
    }
}

impl<K, V> ExtendiableHash<K, V> 
    where K: Hash + Eq + Clone, V: std::fmt::Display + Clone + Copy + Ord
{
    pub fn new(global_depth: usize, bucket_cap: usize) -> Self {
        // 这里保证了每个目录项必有一个桶
        assert!(global_depth > 0);

        let mut entries = vec![];
        for _ in 0..(1<<global_depth) {
            entries.push(Rc::new(RefCell::new(Bucket::new(bucket_cap, global_depth))));
        }
        Self {
            entries,
            global_depth,
            bucket_cap,
        }
    }

    // 目录为空时的插入, 怎么保证目录自动增长?
    // 不可能目录为空, 因为global_depth: usize最小为0, 即会至少有一个桶
    //
    /// 找到相应的桶做插入操作, 当桶满时自动分裂
    pub fn insert(&mut self, key: K, value: V) -> bool {
        let bucket_id = self.hash(&key);
        assert!(bucket_id < 1 << self.global_depth);

        let bucket = self.entries[bucket_id].clone();
        if bucket.borrow().contains_key(&key) {
            return false;
        }

        if bucket.borrow_mut().is_full() {
            self.split(bucket_id);
            self.insert(key, value);
        } else {
            bucket.borrow_mut().insert(key, value);
        }
        true
    }

    /// 更新kv, 不存在时自动插入, 存在时自动覆盖
    pub fn update(&mut self, key: K, value: V) {
        let bucket_id = self.hash(&key);
        let mut bucket = self.entries[bucket_id].borrow_mut();
        bucket.update(key, value);
    }

    /// 3中模式: 
    ///  Merge: 自动合并
    ///  Shrink: 自动合并+自动收缩
    ///  No: 不压缩
    pub fn remove(&mut self, key: &K, mode: Mode) {
        let bucket_id = self.hash(&key);
        let bucket = self.entries[bucket_id].clone();
        bucket.borrow_mut().remove(key);

        match mode {
            Mode::No => {},
            Mode::Merge => self.merge(bucket_id),
            Mode::Shrink => self.shrink(bucket_id),
        }
    }

    pub fn get(&self, key: &K) -> Option<Ref<V>> {
        let bucket_id = self.hash(&key);
        let bucket = self.entries[bucket_id].clone();
        if bucket.borrow().contains_key(&key) {
            Some(Ref::map(self.entries[bucket_id].borrow(), |b| b.get(&key).unwrap()))
        } else {
            None
        }
    }

    pub fn get_mut(&self, key: &K) -> Option<RefMut<V>> {
        let bucket_id = self.hash(&key);
        let bucket = self.entries[bucket_id].borrow();
        if bucket.contains_key(&key) {
            Some(RefMut::map(self.entries[bucket_id].borrow_mut(), |b| b.get_mut(&key).unwrap()))
        } else {
            None
        }
    }

    /// 显示目录项映射关系和内容
    pub fn display(&self) {
        println!("global_depth: {}\n", self.global_depth);
        for i in 0..1<<self.global_depth {
            print!("{}: ", self.bucket_id_string(i));
            self.entries[i].borrow().display();
        }
    }

    pub fn bucket_id_string(&self, bucket_id: usize) -> String {
        let mut str = String::new();
        let mut d = self.entries[bucket_id].borrow().local_depth();
        let mut n = bucket_id;
        while n > 0 && d > 0 {
            str = format!("{}{}", n%2, str);
            n /= 2;
            d -= 1;
        }
        while d > 0 {
            str = format!("0{}", str);
            d -= 1;
        }
        str
    }

    /// 根据全局深度对桶号进行掩码
    ///  global_depth表示使用的bit数
    ///  e.g. 3: 1<<3 = 8 = 1000 而 (1<<3) - 1 = 111
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize & ((1<<self.global_depth)-1)
    }

    /// 对应id的桶需要分裂
    ///  1. 如果local_depth == global_depth, 目录需要扩容
    ///  2. 插入新entry, 并重新映射
    ///  3. 数据迁移: ⭐
    fn split(&mut self, bucket_id: usize) {
        let bucket = self.entries[bucket_id].clone();
        let new_depth = bucket.borrow_mut().depth_up();

        if new_depth > self.global_depth {
            self.grow();
        }

        // 新桶和旧桶关系: 仅最高bit不同
        let pair_bucket_id = bucket.borrow().pair_index(bucket_id);
        self.entries[pair_bucket_id] = Rc::new(RefCell::new(Bucket::new(self.bucket_cap, new_depth)));

        // ⭐计算其他待重新映射的桶⭐
        //
        // 目前已知两独立的桶: bucket_id, new_bucket_id
        // 所有entries中根据后缀重新映射桶
        // 原本是都是映射到bucket_id, 所以现在与new_bucket_id后缀相同的映射需要变动
        // TODO: 试一下后缀算法
        let step = 1 << new_depth;
        for i in (pair_bucket_id..1<<self.global_depth).step_by(step) {
            self.entries[i] = self.entries[pair_bucket_id].clone();
        }
        for i in (0..=pair_bucket_id).rev().step_by(step) {
            self.entries[i] = self.entries[pair_bucket_id].clone();
        }

        // TODO: 数据拷贝开销怎么解决
        // 旧桶中数据重新分配
        let mut kv_store = vec![];
        for (k, v) in bucket.borrow_mut().table_mut() {
            let id = self.hash(k);
            // println!("======{:x} {:x} {:x}== {} {}", id & (1<<new_depth)-1, pair_bucket_id & (1<<new_depth)-1, bucket_id & (1<<new_depth)-1, new_depth, self.global_depth);
            if id & (1<<new_depth)-1 == pair_bucket_id & (1<<new_depth)-1 {
                kv_store.push((k.clone(), v.clone()));
            }
        }

        for (k, v) in kv_store {
            bucket.borrow_mut().table_mut().remove(&k);
            self.insert(k, v);
        }
    }

    /// 当前桶元素删除, 如果删除后桶空则合并
    /// 桶重新映射, 回收空entries
    fn merge(&mut self, bucket_id: usize) {
        let bucket = self.entries[bucket_id].clone();
        // 只有桶为空, local_depth>0时才会有合并
        if !bucket.borrow().is_empty() || bucket.borrow().local_depth() <= 1 {
            return;
        }

        let current_depth = bucket.borrow().local_depth();
        let pair_bucket_id = bucket.borrow().pair_index(bucket_id);
        let pair_bucket = self.entries[pair_bucket_id].clone();
        // 另一半已经扩容, 不能合并
        if pair_bucket.borrow().local_depth() != current_depth {
            return;
        }
        let step = 1 << current_depth;
        // 找到待重新映射的所有桶
        for i in (bucket_id..1<<self.global_depth).step_by(step) {
            self.entries[i] = pair_bucket.clone();
        }
        for i in (0..=bucket_id).rev().step_by(step) {
            self.entries[i] = pair_bucket.clone();
        }
        pair_bucket.borrow_mut().depth_down();
    }

    /// 目录项翻倍, 全局深度增加, 目录项重新映射
    ///  重新映射的方法: 直接从头到位append, 因为二进制翻倍的特定, 后缀刚好能够相同
    fn grow(&mut self) {
        // 使用reserve预留len+additional的空间, 防止频繁分配
        self.entries.reserve(self.entries.len());
        for i in 0..1<<self.global_depth {
            self.entries.push(self.entries[i].clone())
        }
        self.global_depth += 1;
    }

    /// 先merge回收空桶, 再shrink回收目录项
    fn shrink(&mut self, bucket_id: usize) {
        self.merge(bucket_id);

        if self.global_depth == 0 {
            return;
        }

        // 如果存在local_depth == global_depth的目录项, 说明存则使用后半项的桶, 不能压缩
        for b in &self.entries {
            if b.borrow().local_depth() == self.global_depth { return; }
        }

        // 如果不存在local_depth与global_depth相同则说明每个桶至少有两个引用
        // 且其中一个引用在目录项的前半部分中
        // 所以直接弹出后半部分目录项
        self.global_depth -= 1;
        for _ in 0..1<<self.global_depth {
            self.entries.pop();
        }
    }
}

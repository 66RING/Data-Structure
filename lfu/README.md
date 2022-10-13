# lfu

## 哈希表 + Order Set

- 使用有序set保存节点, 根据time和cnt排序
	* freq = cnt / T, T是整个程序是生命周期, 从而就简化成了根据cnt排序
	* 当cnt相同时根据上次使用的时间淘汰最久未使用的(局部性)
- hashmap用来做查找
- rust中用BTreeSet用来做淘汰


### Result

- **rust中自定义结构体的排序方法**
	* 为结构体实现`Ord` trait和`PartialOrd` trait 
	* 而这两个trait又要求元素可比较, 所以使用`#[derive()]`为结构体实现`Eq`和`PartialEq`
	```rust
	impl Ord for Node {
		fn cmp(&self, other: &Self) -> Ordering {
			match self.frequency.cmp(&other.frequency) {
				Ordering::Equal => self.id.cmp(&other.id),
				_o => _o,
			}
		}
	}
	impl PartialOrd for Node {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
		}
	}
	```
- rust中获取迭代器的第一个元素: `T.iter().next()`
- HashMap中元素更新的方法
	* `get_mut()`, `entry().or_insert()`


## 双哈希表

- 一个HashMap做所有kv的索引, 一个HashMap记录各个freq的lru链表
- 记录所有`freq list`中最小的freq: `min_freq`
- 淘汰时从`min_freq`所在的list中淘汰末尾元素
- 使用/更新一个节点时, 更新他的"lfu值"
	* 它的freq++
	* 如果原来所在的freq list空了则删除freq table中的内存
	* 如果原来所在的freq list空了且freq是`min_freq`则更新`min_freq`
- 移动后记得更新链表, erase元素的`prev.next`和`next.prev`

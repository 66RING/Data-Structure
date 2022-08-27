# 跳表

```
1   -->   3      -->     6
|         |              |
v         v              v
1 -> 2 -> 3 -> 4 -> 5 -> 6
```


1. 数据结构
	- 每层用一个链表表示, 链表存在head哨兵 
	- 一个`next`指针表示当前层水平方向的链表
	- 一个`down`指针指向下一层更细粒度
	- rust
		* rust中需要使用Rc, RefCell来构造链表, 来实现多所有权和内部可变性
2. 算法
	- 增:
		1. 从head哨兵开始, 如果next大则表示可能在当前位置插入, 当前节点入栈, down
		2. 如果next小则next找到合适的位置
		3. down到None会遍历完成
		4. 利用保存在栈中的"拐角"节点, 从底层往上插入, 利用随机数决定释放连续插入
	- 删:
		* 遍历next and down, key == target时要down, 防止多删
			+ 即对于重复节点, 我们以第一个为准
		* 检测到目标节点时则将其从链表中删除
	- 查
		* 遍历next and down, 每次down时记录下当前节点
		* 遍历接收后判断最后的节点是否的目标节点
3. rust
	- `Option.filter().is_some()`, 取出Option的内部引用做判断
		* 等价`Option.is_some() && Option.as_ref().unwrap()`
	- `Rc`, 多只读属性的只能指针, 使用`clone()`方法新增引用
	- `RefCell`, 提供内部可变性, 使用`borrow()`, `borrow_mut()`方法取出内部值


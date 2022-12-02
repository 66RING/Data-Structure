# (双)链表

- `push_back()`
- `pop_back()`
- `push_front()`
- `pop_front()`
- `peek_front()`
	* 相当于取出head的值, 注意我们抽象整个list的prev, next等都是为value服务的
- `peek_front_mut()`
- `peek_back()`
	* 相当于取出tail
- `peek_back_mut()`
- `into_iter()`



- rust
	* 如何取出Rc, RefCell中的值, 摆脱Rc和RefCell的束缚
		+ `Rc::try_unwrap(old_head).ok()`返回内部元素
		+ `refcell.into_innter()`销毁RefCell返回内部元素
	* rust中的迭代器
		+ 实现`Iterator` trait, `next()`
		+ 实现`DoubleEndedIterator` trait, `next_bacK()`
	* `Ref`/`RefMut`
		+ 在不销毁`Rc`, `RefCell`的情况下取出内部值, 那只能取出引用
		+ `Ref::map(Ref, || {&val})`, e.g. `Ref::map(node.borrow(), |node| &node.value )`
	* `take()`取出Option, 原来的位置置None
	* **`Rc::try_unwrap()`可以用于防止内存泄露**
		+ 当Rc引用为1时才能返回内部元素


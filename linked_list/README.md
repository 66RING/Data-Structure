# (双)链表


## bad safe dequeue

使用`Rc<RefCell<T>>`安全代码实现, 但是我们只能返回`Ref`, `RefMut`

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


## ok unsafe dequeue

- 对于多重引用问题我们分别新建一个`*mut raw_tail`, 一个`Some(new_tail)`, 互不影响

- 静态分析工具
    * `rustup +nightly-2022-01-21 component add miri`
    * `MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly-2022-01-21 miri test`

### Stacked borrwos规则

```rust
let mut data = 10;
let ref1 = &mut data;
let ref2 = &mut *ref1;

*ref2 += 2;
*ref1 += 1;
```

你不能改变ref1, ref2的顺序。所以对于需要嵌套执行的场景, 为了不违背rust可变借用规则, 你需要像stack一样获取所有权: 你只能操作最新(栈顶)元素。

安全rust代码自动会遵循这个规则, 你可以根据这个规则打败编译器。但问题是一旦你使用unsafe, 你需要多加小心。



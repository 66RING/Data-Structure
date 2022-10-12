# 单向链表(unsafe版)

- 逻辑和c中直接使用指针一致, 主要就是几个api的技巧
- `std::ptr::null_mut()`, `str::ptr::null()`创建空指针
- `Box::into_raw(Box::new(T))`创建裸指针
- `unsafe {Box::from_raw(ptr)}`从裸指针创建一个Box
	* 好处是能够**取出引用**
		+ can not move out of的情况: `return (*self.head).elem`

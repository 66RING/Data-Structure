# 前缀树

https://leetcode.cn/problems/QC3q1f/

- 对参数的trait bound, 抽象
    * `where K: trait1 + trait2`法
        + 全局的
    * `(K: impl T)`法
        + 针对某个方法的
- 类型不匹配可以传入泛型, 如`IntoIterator`默认返回`Item`, 使用`IntoIterator<Item = K>`就可以返回K了
- 不同方法需要的trait不同, 如果直接使用`where`会让方法是使用边严格, 降低易用性
    * 所以: **通用需求的trait使用where, 特定的trait在函数签名处才用impl绑定**

复杂的trait bound/泛型的例子

```rust

impl<Key, Type> Trie<Key, Type>
where
    Key: Default + Eq + Hash,
{
    pub fn get(&self, key: impl IntoIterator<Item = Key>) {}
}
```

要求key是实现了`IntoIterator`的, 然后`IntoIterator`返回的是可以`Hash`可以`Eq`的

区别

```rust

impl<Key, Type> Trie<Key, Type>
{
    pub fn get(&self, key: impl IntoIterator<Item = Key> + Eq + Hash) {}
}
```

这个的要求是key实现`IntoIterator`的, 然后`IntoIterator`返回的是可以是Key, 并且参数`key`还是要有`Eq`和`Hash`的。与上面的区别是, 上面只要求`IntoIterator`的返回可以Hash和Eq

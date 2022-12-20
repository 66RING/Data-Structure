# 跳表

- 参考: https://github.com/withoutboats/kudzu
- 验证: https://leetcode.cn/problems/design-skiplist/solution/

- tips
    * 有上层节点就一定有下层节点; 有下层节点, 但是可能高度原因, 没有上层节点
        + 所以get时可以equal就返回


- link list有很好的RCU潜质, 但是lock free != db封锁封锁等级


## rust tips

- 烦人的内部可变性问题
    * 可以直接一步到位使用`AtomicPtr<T>`做内部可变性
    * 否则就是`Rc<RefCell<T>>`, 或者`RefCell<Box<T>>`
        + 但是使用了`RefCell`就无法返回纯真的`&T`了
- 提前返回处理, e.g. `if a == b { return }`
    * 我们只希望`a != b`这中特殊情况下返回, 其他所有情况都不返回
    * 但是因为rust常常又有Option手写if else一不小心就会漏掉None case, 所以可以使用`if map_or(true, a == b)`
- `&a.unwrap()` != `a.as_ref().unwrap()`, 后者生命周期可能会更长, 因为内部数据的生命周期可能更长
- `RefCell`中的`borrow()`会将所有权拿到内部`Ref`, 就像`Option`的`as_ref()`会改为内部引用一样
    * 所以如果`match a.borrow().elem`拿数据后还想用a, 就可以先`a.clone().borrow()`
- `match a.borrow().elem`会拿着a不放, `if a.borrow().elem`就不会, 因为match有一个模式匹配的过程, 相当于新建变量
- `.as_ref().cloned()`配合Rc
- `iter().enumerate().take(n)`


## struct

- skiplist
    * `lanes[level]`: 链表数组, 每层都有对应的next指针
    * height: 当前规模
    * 这个跳表的实现就是固定高度(层数)的链表lanes
    * 高度动态扩展, 使用一个height标记当前规模
    * 使用原子指针串联, 方便无锁更新
    * 链表数组从底向上扩展, 最底层就是所有元素, 最底层为第一层
- Node
    * inner: 内部数据
    * `next[level]/lanes[level]`: 链表
    * `Node::new(max_height)`, 根据最大构造建立一个随机高度的塔
    * `layout`: TODO: `kudzu`的难点


- new
    * 初始化
- get
    * 逐层`node = node[level].next`查找最大小于target元素
        + 参考`kudzu`的实现可以优雅地找最大小于
- insert
    * 逐层`node = node[level].next`查找最大小于target元素
    * 记录每层的最大小于target的元素, target将要插入再他们后面
        + 即down的时候要记录一下: `kudzu`中记录的数组名为`spots`, 它记录了待插入位置的前后和后继
    * 申请一塔节点作为待插入节点, 高度使用概率算法确定
    * 自底向上插入, 建塔
- remove
    * 类似insert, 则层查找最大小于target的元素
    * 记录每层的最大小于元素`update[]`, **这些元素是待删除节点的前驱**
    * 如果`update[i].next`是当前目标节点则可以删除
    * 最后如果最上层节点只有一个(即head.next = null), 则可以压缩层高, 下次查找更加快速
- nodes: 返回最底层链表, 即可获得所有元素
- dealloc
    * `kudzu`的实现非常复杂
- ⭐ 随机层高算法
    * bitmap模拟, 末尾几个连续的0就是几层

- TODO: memory ordering things
    * Relaxed
    * ...

## 魔鬼细节

- RCU & 插入建塔: 
        1. 从底向上插入, 不至于上面查到了下面没有
        2. RCU插入时, 先`new_node.next = succ`防止并发访问到`new_node`时没有后继
- ⭐ 插入时不需要从0..self.curr_height建塔
    * 因为如果new_height > curr_height时, 则自然会新建层如果new_height <= curr_height时, new_height就是要新建的随机高度



##  paper

- https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&ved=2ahUKEwie9JDblID8AhXZq1YBHenJBysQFnoECAcQAQ&url=https%3A%2F%2Fcs.brown.edu%2Fpeople%2Fmph%2FLevHLS06%2FOPODIS2006-BA.pdf&usg=AOvVaw2_z-sdLhhFpEPXJBmtCCLF

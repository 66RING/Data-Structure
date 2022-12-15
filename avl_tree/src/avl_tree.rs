use std::cmp::{Ord, max};
use std::mem;
use std::ops::Not;

struct AVLNode<T: Ord> {
    value: T,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
    height: usize,
}

struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    length: usize,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<T: Ord> AVLNode<T> {

    // 平衡操作都是被dfs调用的, 所以我们可以只关系一层
    pub fn rebalance(&mut self) {
        // 重新计算高度
        self.update_height();
        let factor = self.balance_factor();
        if factor != 2 && factor != -2 {
            // 不用更新
            return;
        }
        let (side, subtree) = if factor < 0 {
            // 如果右子树更高则左旋, 可能准备对右子树旋转
            (Side::Left, self.right.as_mut().unwrap())
        } else {
            // 如果左子树更高则右旋, 可能准备对左子树旋转
            (Side::Right, self.left.as_mut().unwrap())
        };

        let sub_factor = subtree.balance_factor();
        if factor < 0 && sub_factor > 0 {
            // RL的情况
            subtree.rotate(Side::Left);
        } else if factor > 0 && sub_factor < 0 {
            // LR的情况
            subtree.rotate(Side::Right);
        }
        self.rotate(side);
    }

    /// ⭐⭐⭐
    /// 对子树旋转
    ///  1. 孩子继承: 旋转方向的"反向子树"的"同向孩子"需要继承
    ///  2. 新根节点: 旋转方向的"反向子树"称为新根
    ///  3. 旧根成子树: 旧根成为新根的"同向孩子"
    fn rotate(&mut self, side: Side) {
        let mut subtree = self.child_mut(!side).take().unwrap();
        let new_child = subtree.child_mut(side).take();
        *self.child_mut(!side) = new_child;
        self.update_height();

        // 根节点交换
        mem::swap(self, subtree.as_mut());

        // 旧跟成子树
        *self.child_mut(side) = Some(subtree);
        self.update_height();
    }

    fn child(&self, side: Side) -> &Option<Box<AVLNode<T>>>{
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }
    fn child_mut(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>>{
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.height(Side::Left), self.height(Side::Right));
    }

    /// 计算l - r高度差
    /// WARN: 小心无符号溢出: 0 - 1, 所以全部转大减小
    fn balance_factor(&self) -> isize {
        let (left, right) = (self.height(Side::Left), self.height(Side::Right));
        if left < right {
            -((right - left) as isize)
        } else {
            (left - right) as isize
        }
    }

    /// Returns the height of the left or right subtree.
    fn height(&self, side: Side) -> usize {
        self.child(side).as_ref().map_or(0, |n| n.height)
    }
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        Self { root: None, length: 0 }
    }
    pub fn insert(&mut self, value: T) -> bool {
        let inserted = Self::tree_insert(&mut self.root, value);
        if inserted {
            self.length += 1;
        }
        inserted
    }

    fn tree_insert(tree: &mut Option<Box<AVLNode<T>>>, value: T) -> bool {
        if let Some(node) = tree {
            let inserted = match value.cmp(&node.value) {
                std::cmp::Ordering::Less => Self::tree_insert(&mut node.left, value),
                std::cmp::Ordering::Equal => false,
                std::cmp::Ordering::Greater => Self::tree_insert(&mut node.right, value),
            };
            if inserted {
                node.rebalance();
            }
            inserted
        } else {
            *tree = Some(Box::new(AVLNode {
                value,
                left: None,
                right: None,
                height: 1,
            }));
            true
        }
    }

    // 二叉搜索, 找相等
    pub fn contains(&self, value: &T) -> bool {
        let mut tree = &self.root;
        while let Some(node) = tree {
            tree = match value.cmp(&node.value) {
                std::cmp::Ordering::Less => &node.left,
                std::cmp::Ordering::Greater => &node.right,
                // 找到
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let removed = Self::tree_remove(&mut self.root, value);
        if removed {
            self.length -= 1;
        }
        removed
    }

    // 删除value对应节点
    //  1. 如果是非叶子节点则需要考虑孩子的领养问题: merge
    //  2. 如果节点存在且产生了删除, 需要考虑dfs做平衡
    fn tree_remove(tree: &mut Option<Box<AVLNode<T>>>, value: &T) -> bool {
        if let Some(node) = tree {
            let removed = match value.cmp(&node.value) {
                // 如果待删节点大, 则递归查找右子树
                std::cmp::Ordering::Less => Self::tree_remove(&mut node.left, value),
                std::cmp::Ordering::Greater => Self::tree_remove(&mut node.right, value),
                std::cmp::Ordering::Equal => {
                    // 判断是否存在左右孩子需要领养
                    *tree = match (node.left.take(), node.right.take()) {
                        // 不需要领养: 直接删除当前节点
                        (None, None) => None,
                        // 只需要领养一边
                        (None, Some(n)) | (Some(n), None) => Some(n),
                        (Some(l), Some(r)) => Some(Self::merge(l, r)),
                    };
                    return true;
                }
            };
            if removed {
                node.rebalance();
            }
            removed
        } else {
            false
        }
    }

    fn merge(left: Box<AVLNode<T>>, right: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut new_right = Some(right);
        let mut new_root = Self::take_min(&mut new_right).unwrap();
        new_root.left = Some(left);
        new_root.right = new_right;
        new_root.rebalance();
        new_root
    }


    /// take掉最左(小)节点
    ///  找左子树的最左节点
    ///  如果左子树不存在, 则当前节点就行最小, 树根变为右节点
    ///  同理remove, take掉后相当于删除, 需要dfs平衡
    pub fn take_min(tree: &mut Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>>{
        if let Some(mut node) = tree.take() {
            if let Some(small) = Self::take_min(&mut node.left) {
                // 尝试从左子树找
                node.rebalance();
                // 所有权重新接上
                *tree = Some(node);
                Some(small)
            } else {
                // 并让右子树占据当前节点
                // 如果都不存在可以直接返回当前节点了
                *tree = node.right.take();
                Some(node)
            }
        } else {
            None
        }
    }

    /// 先序遍历预处理: dfs到底, 左节点压栈
    /// Returns an iterator that visits the nodes in the tree in order.
    fn node_iter(&self) -> NodeIter<T> {
        let cap = self.root.as_ref().map_or(0, |n| n.height);
        let mut node_iter = NodeIter {
            stack: Vec::with_capacity(cap),
        };

        // Initialize stack with path to leftmost child
        let mut child = &self.root;
        while let Some(node) = child {
            node_iter.stack.push(node.as_ref());
            child = &node.left;
        }
        node_iter
    }

    /// 迭代器转换
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node_iter: self.node_iter(),
        }
    }

    /// Returns the number of values in the tree.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if the tree contains no values.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}


impl<T: Ord> Default for AVLTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 迭代器转AVL树
impl<T: Ord> FromIterator<T> for AVLTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = AVLTree::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

/// An iterator over the nodes of an `AVLTree`.
///
/// This struct is created by the `node_iter` method of `AVLTree`.
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a AVLNode<T>;

    /// 先序遍历
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            // Push left path of right subtree to stack
            let mut child = &node.right;
            while let Some(subtree) = child {
                self.stack.push(subtree.as_ref());
                child = &subtree.left;
            }
            Some(node)
        } else {
            None
        }
    }
}

/// An iterator over the items of an `AVLTree`.
///
/// This struct is created by the `iter` method of `AVLTree`.
pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.node_iter.next() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::AVLTree;

    /// Returns `true` if all nodes in the tree are balanced.
    fn is_balanced<T: Ord>(tree: &AVLTree<T>) -> bool {
        tree.node_iter()
            .all(|n| (-1..=1).contains(&n.balance_factor()))
    }

    #[test]
    fn len() {
        let tree: AVLTree<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: AVLTree<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = AVLTree::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: AVLTree<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
        let tree: AVLTree<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().map(|&x| x)));
    }

    #[test]
    fn balanced() {
        let mut tree: AVLTree<_> = (1..8).collect();
        assert!(is_balanced(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_balanced(&tree));
        }
    }
}

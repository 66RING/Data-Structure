pub struct BinarySearchTree<T> {
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
    value: Option<T>,
}

impl<T> BinarySearchTree<T> 
where T: Ord,
{
    pub fn new() -> Self {
        Self { left: None, right: None, value: None }
    }

    pub fn search(&self, val: &T) -> bool {
        match &self.value {
            Some(v) => {
                match v.cmp(val) {
                    std::cmp::Ordering::Less =>  match &self.right {
                        Some(node) => node.search(val),
                        None => false
                    }
                    std::cmp::Ordering::Equal => true,
                    std::cmp::Ordering::Greater => match &self.left {
                        Some(node) => node.search(val),
                        None => false
                    }
                }
            }
            None => false
        }
    }

    pub fn insert(&mut self, val: T)  {
        if self.value.is_none() {
            self.value = Some(val);
            return
        }
        if let Some(v) = &self.value {
            let target = if *v > val {
                // 如果小于当前节点, 差插入左子树
                &mut self.left
            } else {
                // 如果大于当前节点, 差插入右子树
                &mut self.right
            };
            match target {
                Some(node) => node.insert(val),
                None => {
                    let mut new_node = BinarySearchTree::new();
                    new_node.value = Some(val);
                    // TIPS: 直接操作引用, 原地修改left/right
                    *target = Some(Box::new(new_node));
                }
            }
        }
    }

    /// 查找小于等于value的最大节点
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(v) => match v.cmp(value) {
                std::cmp::Ordering::Less => match &self.right {
                    // 比当前节点大, 进入右子树看看 **有没有更大的**, 至少当前节点是可以的
                    Some(node) => {
                        let val = node.floor(value);
                        match val {
                            // 如果有更大的就用更大的
                            Some(_) => val,
                            // 如果有更大的就用当前节点了
                            None => Some(v),
                        }
                    }
                    // 如果右子树不存在, 那就不可能存在更大的大小value的节点了, 返回当前节点值
                    None => Some(v),
                }
                std::cmp::Ordering::Equal => Some(v),
                std::cmp::Ordering::Greater => match &self.left {
                    // 比当前节点小, 进入左子树找小于的
                    // 直接递归, 因为不用考虑当该前节点
                    Some(node) => node.floor(value),
                    // 如果左子树不存在则说明不存在小于的节点了, 取floor失败
                    None => None,
                }
            }
            None => None,
        }
    }

    /// 查找大于等于value的最小节点
    pub fn ceil(&self, key: &T) -> Option<&T> {
        match &self.value {
            Some(v) => match v.cmp(key) {
                std::cmp::Ordering::Less => match &self.right {
                    // 如果比当前节点大, 则查右子树看看有没有能大于key的
                    Some(node) => node.ceil(key),
                    // 如果右子树不存在则说明没有大于key的的节点
                    None => None
                }
                std::cmp::Ordering::Equal => Some(v),
                std::cmp::Ordering::Greater => match &self.left {
                    // 如果比当前节点小, 那至少存在当前节点可以做ceil
                    // 看看左子树有没有更小的大于key的节点
                    Some(node) => {
                        let val = node.ceil(key);
                        match val {
                            Some(_) => val,
                            None => Some(v),
                        }
                    }
                    // 如果没有左子树, 则没有更小的大于key的了, 直接使用当前节点
                    None => Some(v),
                }
            }
            None => None,
        }
    }

    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => match &self.value {
                Some(v) => Some(v),
                None => None,
            }
        }
    }

    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => match &self.value {
                Some(v) => Some(v),
                None => None,
            }
        }
    }

    pub fn iter(&self) -> BinarySearchTreeIter<T> {
        BinarySearchTreeIter::new(self)
    }
}

pub struct BinarySearchTreeIter<'a, T> {
    stack: Vec<&'a BinarySearchTree<T>>
}

impl<'a, T> BinarySearchTreeIter<'a, T> {
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        // 二叉树的先序遍历, 初始先先递归调用(即压栈)
        //  dfs(root.left)
        //  root
        //  dfs(root.right)
        iter.dfs();
        iter
    }

    pub fn dfs(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            // TIPS: &Box<T>取*会自动两次解引用
            //  如果不是返回as_ref引用的话, 临时变量rchild会在函数结束时自动销毁
            if let Some(rchild) = node.right.as_ref() {
                self.stack.push(rchild);
                self.dfs();
            }
            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}


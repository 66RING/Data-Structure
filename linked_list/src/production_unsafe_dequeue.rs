#![allow(dead_code)]
use std::marker::PhantomData;
use std::ptr::{NonNull, self};

// 完整版见: https://rust-unofficial.github.io/too-many-lists/sixth-combinatorics.html
// 传递一个unsafe的思路尔

type Link<T> = Option<NonNull<Node<T>>>;

struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _foo: PhantomData<T>,
}

struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _foo: PhantomData,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|old_head|{
                // 因为我们插入是Box插入的，限制取会, 然后最后自动删除
                //  tips: 开头就申请内存, 即使OOM错误了, 我们的程序也不会造成忘记删节点前后继的问题
                let node = Box::from_raw(old_head.as_ptr());
                let result = node.elem;

                // 新head成立
                self.head = node.next;
                if let Some(new_head) = self.head {
                    (*new_head.as_ptr()).prev = None;
                } else {
                    // 如果新head空了, 说明链表空了
                    self.tail = None;
                }
                self.len -= 1;
                result
            })
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                next: None,
                prev: None,
                elem,
            })));
            if let Some(old_node) = self.head {
                (*old_node.as_ptr()).prev = Some(new_node);
                (*new_node.as_ptr()).next = Some(old_node);
            } else {
                self.tail = Some(new_node);
            }
            self.len += 1;
            self.head = Some(new_node);
        }
    }
}


#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}

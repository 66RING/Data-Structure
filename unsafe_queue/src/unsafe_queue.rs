use std::ptr;

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node { elem, next: ptr::null_mut() }
    }
}

struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    // 插入head, 必要时对tail操作
    pub fn push_back(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node::new(elem)));
        if self.tail.is_null() {
            // 插入第一个元素, 初始化head
            self.head = new_tail
        } else {
            unsafe { (*self.tail).next = new_tail; }
        }

        // push back必会更新tail
        self.tail = new_tail;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let old_head = self.head;
            // 为什么要转成Box而不是直接(*self.head).next
            //  因为最后要利用这个Box返回
            //  否则就unsafe包裹的裸指针解引用返回
            let head = unsafe { Box::from_raw(self.head) };
            self.head = head.next;
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            Some(head.elem)
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basic() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn iter() {
    }

    #[test]
    fn iter_mut() {
    }
}

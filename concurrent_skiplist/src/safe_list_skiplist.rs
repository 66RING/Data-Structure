use rand::{thread_rng, Rng};
use std::rc::Rc;
use std::cell::RefCell;


struct Node<T, U> {
    key: T,
    value: U,
    // linked list
    next: Option<Rc<RefCell<Node<T, U>>>>,
    down: Option<Rc<RefCell<Node<T, U>>>>,
}

impl<T, U> Node<T, U> {
    fn new(key: T, value: U) -> Self {
        Self {
            key,
            value,
            next: None,
            down: None,
        }
    }
}

pub struct Skiplist {
    head: Option<Rc<RefCell<Node<i32, i32>>>>,
}

impl Skiplist {
    pub fn new() -> Self {
        Self {
            head: Some(Rc::new(RefCell::new(Node::new(-1, -1)))),
        }
    }

    // 能右就右: key < target
    // 不能就下
    // 需要记录上一个down的节点, 这里用Vec保存所有
    pub fn search(&self, target: i32) -> bool {
        let mut curr = self.head.clone();
        let mut qu = Vec::new();
        // 遍历, 向next找到最接近的范围, 再down提升精度
        while let Some(node) = curr {
            curr = if node.borrow().next.clone().filter(|n| n.borrow().key <= target).is_some() {
                node.borrow().next.clone()
            } else {
                qu.push(node.clone());
                node.borrow().down.clone()
            }

        }
        qu.last().filter(|n| n.borrow().key == target).is_some()
    }

    pub fn add(&mut self, target: i32) {
        let mut curr = self.head.clone();
        // 所有拐点保存到栈中, 最后逐层插入
        let mut qu = Vec::new();
        let mut is_insert = true;
        let mut down = None;
        while let Some(node) = curr {
            curr = if node.borrow().next.clone().filter(|n| n.borrow().key <= target).is_some() {
                node.borrow().next.clone()
            } else {
                qu.push(node.clone());
                node.borrow().down.clone()
            }

        }
        // 弹出栈中记录的拐点, 逐层插入, 利用随机数判断是否插入从而保证均匀性
        while is_insert && !qu.is_empty() {
            let node = qu.pop().unwrap();
            let new_node = Rc::new(RefCell::new(Node::new(target, target)));
            // 插入链表
            new_node.borrow_mut().next = node.as_ref().borrow().next.clone();
            new_node.borrow_mut().down = down;
            node.borrow_mut().next = Some(new_node.clone());
            down = Some(new_node.clone());
            is_insert = thread_rng().gen_range(0.0f64..1.0f64) < 0.5;
        }
        // 若随机到一直插入直到顶部, 则在顶部新增层
        if is_insert {
            let new_node = Rc::new(RefCell::new(Node::new(-1, -1)));
            new_node.borrow_mut().down = self.head.clone();
            self.head = Some(new_node);
        }
    }

    pub fn erase(&self, target: i32) -> bool {
        // 找到所有down, 保存到栈中, 则栈顶就是目标
        let mut curr = self.head.clone();
        let mut is_found = false;
        while let Some(node) = curr {
            // 遍历 key == target时要down防止多删
            curr = if node.borrow().next.is_some() && node.borrow().next.as_ref().unwrap().borrow().key < target {
                node.borrow().next.clone()
            } else {
                node.borrow().down.clone()
            };

            // curr已经next或down, 可以安全删除
            if node.borrow().next.is_some() && node.borrow().next.as_ref().unwrap().borrow().key == target {
                is_found = true;
                let next = node.borrow().next.as_ref().unwrap().borrow_mut().next.clone();
                node.borrow_mut().next = next;
            }

        }
        is_found
    }
}

#[test]
fn skiplist() {
    let mut sl = Skiplist::new();
    sl.add(1);
    assert_eq!(sl.search(1), true);
    assert_eq!(sl.erase(1), true);
    assert_eq!(sl.search(1), false);

    sl.add(2);
    sl.add(2);
    assert_eq!(sl.erase(2), true);
    assert_eq!(sl.search(2), true);
    assert_eq!(sl.erase(2), true);
    assert_eq!(sl.search(2), false);

    sl.add(1);
    sl.add(2);
    sl.add(3);
    assert_eq!(sl.search(4), false);
    assert_eq!(sl.erase(4), false);
    assert_eq!(sl.erase(2), true);
    assert_eq!(sl.search(2), false);
}


fn main() { }

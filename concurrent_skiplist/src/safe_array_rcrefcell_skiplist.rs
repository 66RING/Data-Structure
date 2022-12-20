use std::cmp::Ordering::*;

use std::rc::Rc;
use std::cell::RefCell;

const MAX_HEIGHT: usize = 32;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    next: Vec<Link>,
    elem: i32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem && self.next.len() == other.next.len()
    }
}

pub struct Skiplist {
    head: Rc<RefCell<Node>>,
    current_height: usize,
}

impl Node {
    pub fn new(elem: i32) -> Self {
        Self {
            next: vec![None; MAX_HEIGHT],
            elem,
        }
    }
}

impl Skiplist {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(Node::new(0)));
        Self {
            // 哨兵
            head,
            current_height: 0,
        }
    }

    pub fn search(&mut self, target: i32) -> bool {
        let mut curr_node = self.head.clone();

        for height in (0..self.current_height).rev() {
            // 如果next存在
            while let Some(next) = curr_node.clone().borrow().next[height].clone() {
                match next.clone().borrow().elem.cmp(&target) {
                    Less => {
                        curr_node = next;
                    },
                    // 因为上层存在则下层必定存在
                    Equal => return true,
                    Greater => break,
                }
            }
        }
        false
    }
    
    pub fn add(&mut self, target: i32) {

        let mut curr_node = self.head.clone();
        let mut update = vec![self.head.clone(); MAX_HEIGHT];

        for height in (0..self.current_height).rev() {
            while let Some(next) = curr_node.clone().borrow().next[height].clone() {
                if next.clone().borrow().elem >= target {
                    break;
                }
                curr_node = next;
            }
            update[height]= curr_node.clone();
        }

        let rand_height = Self::rand_height();
        self.current_height = self.current_height.max(rand_height);
        let new_node = Rc::new(RefCell::new(Node::new(target)));

        // 自底向上插入
        // 不需要更新0..self.curr_height, 因为
        //  如果new_height > curr_height时, 则自然会新建层
        //  如果new_height <= curr_height时, new_height就是要新建的随机高度
        for (i, spot) in update.iter().enumerate().take(rand_height) {
            new_node.borrow_mut().next[i] = spot.borrow_mut().next[i].clone();
            spot.borrow_mut().next[i] = Some(new_node.clone());
        }
    }
    
    pub fn erase(&mut self, target: i32) -> bool {
        let mut curr_node = self.head.clone();
        let mut update = vec![self.head.clone(); MAX_HEIGHT];

        for height in (0..self.current_height).rev() {
            while let Some(next) = curr_node.clone().borrow().next[height].clone() {
                if next.borrow().elem >= target {
                    break;
                }
                curr_node = next;
            }
            update[height]= curr_node.clone();
        }

        // 如果目标节点不存在则删除失败
        if curr_node.borrow().next[0].as_ref().map_or(true, |x| x.borrow().elem != target) {
            return false;
        }
        // 自顶向下拆
        curr_node = curr_node.clone().borrow().next[0].clone().unwrap();
        for i in (0..self.current_height).rev() {
            if update[i].clone().borrow().next[i] == Some(curr_node.clone()) {
                update[i].borrow_mut().next[i] = curr_node.borrow_mut().next[i].clone()
            }
        }

        // 当顶层为空时高度下降
        while self.current_height > 0 && self.head.borrow_mut().next[self.current_height-1].is_none() {
            self.current_height -= 1;
        }

        true
    }

    fn rand_height() -> usize {
        let x = rand::random::<u32>() | 1 << (MAX_HEIGHT - 1);
        1 + x.trailing_zeros() as usize
    }

    pub fn display(&self) {
        for i in (0..self.current_height).rev() {
            let mut curr = self.head.clone();
            while let Some(node) = curr.clone().borrow().next[i].clone() {
                print!("{} ", node.borrow().elem);

                curr = node;
            }
            println!();
        }
    }
}


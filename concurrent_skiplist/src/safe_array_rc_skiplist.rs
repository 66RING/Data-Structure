use std::ptr::{NonNull, self};
use std::cmp::Ordering::*;

use std::rc::Rc;
use std::cell::RefCell;

const MAX_HEIGHT: usize = 32;

type Link = Option<Rc<Node>>;

struct Node {
    next: Vec<Link>,
    elem: i32,
}

pub struct Skiplist {
    head: Rc<Node>,
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
        let head = Rc::new(Node::new(0));
        Self {
            // 哨兵
            head,
            current_height: 0,
        }
    }

    pub fn search(&mut self, target: i32) -> bool {
        let mut curr_node = &self.head;

        for height in (0..self.current_height).rev() {
            // 如果next存在
            while let Some(next) = curr_node.next[height].as_ref() {
                // 且next < target
				match next.elem.cmp(&target) {
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

        let mut curr_node = &self.head;
        let mut update = vec![&self.head; MAX_HEIGHT];

        for height in (0..self.current_height).rev() {
            while let Some(next) = curr_node.next[height].as_ref() {
                if next.elem >= target {
                    break;
                }
                curr_node = next;
            }
            update[height]= curr_node;
        }

        let rand_height = Self::rand_height();
        self.current_height = self.current_height.max(rand_height);
        let new_node = Rc::new(Node::new(target));

        unsafe {
            // 自底向上插入
            // 不需要更新0..self.curr_height, 因为
            //  如果new_height > curr_height时, 则自然会新建层
            //  如果new_height <= curr_height时, new_height就是要新建的随机高度
            for (i, spot) in update.iter().enumerate().take(rand_height) {
                (*(Rc::as_ptr(&new_node) as *mut Node)).next[i] = spot.next[i].as_ref().cloned();
                (*(Rc::as_ptr(spot) as *mut Node)).next[i] = Some(new_node.clone());
            }

        }
    }
    
    pub fn erase(&mut self, target: i32) -> bool {
        let mut curr_node = &self.head;
        let mut update = vec![&self.head; MAX_HEIGHT];

        for height in (0..self.current_height).rev() {
            while let Some(next) = curr_node.next[height].as_ref() {
                if next.elem >= target {
                    break;
                }
                curr_node = next;
            }
            update[height]= curr_node;
        }

        // 如果目标节点不存在则删除失败
        if curr_node.next[0].as_ref().map_or(true, |x| x.elem != target) {
            return false;
        }

        // 自顶向下拆
        for i in (0..self.current_height).rev() {
            if let Some(n) = &update[i].next[i] {
                if n.elem != target {
                    continue;
                }
                // cloned() 相当于 .map(|x|x.clone());
                unsafe { (*(Rc::as_ptr(update[i]) as *mut Node)).next[i] = n.next[i].as_ref().cloned(); }
            }
        }

        // 当顶层为空时高度下降
        while self.current_height > 0 && self.head.next[self.current_height-1].is_none() {
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
            let mut curr = &self.head;
            while let Some(node) = curr.next[i].as_ref() {
                print!("{} ", node.elem);

                curr = node;
            }
            println!();
        }
    }
}


use std::ptr::{NonNull, self};
use std::cmp::Ordering::*;

const MAX_HEIGHT: usize = 32;

type Link = Option<NonNull<Node>>;

struct Node {
    next: [Link; MAX_HEIGHT],
    elem: i32,
}

pub struct Skiplist {
    head: NonNull<Node>,
    current_height: usize,
}

impl Node {
    pub fn new(elem: i32) -> Self {
        Self {
            next: [None; MAX_HEIGHT],
            elem,
        }
    }
}

impl Skiplist {
    pub fn new() -> Self {
        unsafe {
        let head = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(0))));
        Self {
            // 哨兵
            head,
            current_height: 0,
        }
        }
    }

    pub fn search(&mut self, target: i32) -> bool {
        unsafe {

        let mut curr_node = self.head.as_ref();
        let current_height = self.current_height;

        for height in (0..current_height).rev() {
            // 如果next存在
            while let Some(next) = curr_node.next[height] {
                // 且next < target
				match next.as_ref().elem.cmp(&target) {
					Less => curr_node = next.as_ref(),
                    // 因为上层存在则下层必定存在
					Equal => return true,
					Greater => break,
				}
            }
        }
        false

        }
    }
    
    pub fn add(&mut self, target: i32) {
        unsafe {

        let mut curr_node = self.head;
        let mut update = [self.head.as_ptr(); MAX_HEIGHT];

        for height in (0..self.current_height).rev() {
            while let Some(next) = curr_node.as_mut().next[height] {
                if next.as_ref().elem >= target {
                    break;
                }
                curr_node = next;
            }
            update[height]= curr_node.as_ptr();
        }

        let rand_height = Self::rand_height();
        self.current_height = self.current_height.max(rand_height);
        let mut new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(target))));

        // 自底向上插入
        // 不需要更新0..self.curr_height, 因为
        //  如果new_height > curr_height时, 则自然会新建层
        //  如果new_height <= curr_height时, new_height就是要新建的随机高度
        for (i, spot) in update.iter().enumerate().take(rand_height) {
            new_node.as_mut().next[i] = (**spot).next[i];
            (**spot).next[i] = Some(new_node);
        }

        }
    }
    
    pub fn erase(&mut self, target: i32) -> bool {
        unsafe {

        let mut curr_node = self.head;
        let mut update = [self.head.as_ptr(); MAX_HEIGHT];

        for height in (0..self.current_height).rev() {
            while let Some(next) = curr_node.as_mut().next[height] {
                if next.as_ref().elem >= target {
                    break;
                }
                curr_node = next;
            }
            update[height]= curr_node.as_ptr();
        }

        // 如果目标节点不存在则删除失败
        if curr_node.as_ref().next[0].map_or(true, |x| x.as_ref().elem != target) {
            return false;
        }

        // 自顶向下拆
        for i in (0..self.current_height).rev() {
            if let Some(mut n) = (*update[i]).next[i] {
                if n.as_ref().elem != target {
                    continue;
                }

                (*update[i]).next[i] = n.as_mut().next[i];
            }
        }

        // 当顶层为空时高度下降
        while self.current_height > 0 && self.head.as_ref().next[self.current_height-1].is_none() {
            self.current_height -= 1;
        }

        true
        }
    }

    fn rand_height() -> usize {
        let x = rand::random::<u32>() | 1 << (MAX_HEIGHT - 1);
        1 + x.trailing_zeros() as usize
    }

    pub fn display(&self) {
        unsafe {

        for i in (0..self.current_height).rev() {
            let mut curr = self.head;
            while let Some(node) = curr.as_ref().next[i] {
                print!("{} ", node.as_ref().elem);

                curr = node;
            }
            println!();
        }

        }
    }
}


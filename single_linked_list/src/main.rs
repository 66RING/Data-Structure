struct Link<T> {
    value: T,
    next: Option<Box<Link<T>>>,
}

struct List<T: Copy> {
    head: Option<Box<Link<T>>>,
}

impl<T: Copy> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    // 获取index处值
    fn get(&self, index: i32) -> Option<T> {
        let mut p = &self.head;
        let mut cnt = 0;
        while let Some(node) = p {
            if cnt == index {
                return Some(node.value);
            }
            cnt += 1;
            p = &node.next;
        }
        None
    }

    fn add_at_head(&mut self, value: T) {
        self.head = Some(Box::new(Link {
            value,
            next: self.head.take(),
        }));
    }

    fn add_at_tail(&mut self, value: T) {
        let mut p = &mut self.head;
        while let Some(node) = p {
            p = &mut node.next;
        }
        *p = Some(Box::new(Link { value, next: None }));
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        if index == 0 {
            if let Some(node) = &mut self.head {
                self.head = node.next.take();
            }
        }
        let mut p = &mut self.head;
        let mut cnt = 0;
        while let Some(node) = p {
            if cnt + 1 == index {
                let old = node.next.take();
                if let Some(t) = old {
                    node.next = t.next;
                }
                break;
            }
            cnt += 1;
            p = &mut node.next;
        }
    }

    fn add_at_index(&mut self, index: i32, value: T) {
        if index <= 0 {
            self.add_at_head(value);
        } else {
            let mut p = &mut self.head;
            let mut cnt = 0;
            while let Some(node) = p {
                if cnt + 1 == index {
                    node.next = Some(Box::new(Link {
                        value,
                        next: node.next.take(),
                    }));
                    break;
                }
                cnt += 1;
                p = &mut node.next;
            }
        }
    }
}

fn main() {
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basic() {
        let mut obj = List::new();
        obj.add_at_head(1);
        // 1
        obj.add_at_tail(3);
        // 1 3
        obj.add_at_index(1, 2);
        // 1 2 3
        assert_eq!(Some(1), obj.get(0));
        assert_eq!(Some(2), obj.get(1));
        assert_eq!(Some(3), obj.get(2));


        let mut obj = List::new();
        obj.add_at_head(1);
        // 1
        obj.add_at_tail(3);
        // 1 3
        obj.add_at_index(1, 2);
        // 1 2 3
        assert_eq!(Some(2), obj.get(1));
        obj.delete_at_index(1);
        assert_eq!(Some(3), obj.get(1));


        let mut obj = List::new();
        obj.add_at_head(1);
        // 1
        obj.add_at_tail(3);
        // 1 3
        obj.add_at_index(1, 2);
        // 1 2 3
        assert_eq!(Some(2), obj.get(1));
        obj.delete_at_index(0);
        assert_eq!(Some(2), obj.get(0));


        let mut obj = List::new();
        obj.add_at_index(0, 20);
        // 20
        obj.add_at_index(1, 30);
        // 20 30
        assert_eq!(Some(20), obj.get(0));
        assert_eq!(Some(30), obj.get(1));
    }

}


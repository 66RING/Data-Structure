struct Node {
    lazy: bool,
    value: bool,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Self { lazy: false, value: false, left: None, right: None }
    }
}

struct RangeModule {
    root: Node,
}

const N: i32 = 1000000000;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {

    fn new() -> Self {
        Self { root: Node::new() }
    }
    
    fn add_range(&mut self, left: i32, right: i32) {
        Self::update(&mut self.root, 1, N, left, right-1, true);
    }
    
    fn query_range(&mut self, left: i32, right: i32) -> bool {
        Self::query(&mut self.root, 1, N, left, right-1)
    }
    
    fn remove_range(&mut self, left: i32, right: i32) {
        Self::update(&mut self.root, 1, N, left, right-1, false);
    }

    fn update(node: &mut Node, start: i32, end: i32, l: i32, r: i32, value: bool) {
        if l <= start && end <= r {
            node.value = value;
            node.lazy = true;
            return;
        }
        let mid = start/2 + end/2;
        Self::pushDown(node);
        if l <= mid {
            Self::update(node.left.as_mut().unwrap(), start, mid, l, r, value);
        }
        if r > mid {
            Self::update(node.right.as_mut().unwrap(), mid+1, end, l, r, value);
        }
        // dfs回溯更新
        Self::pushUp(node);
    }
    fn query(node: &mut Node, start: i32, end: i32, l: i32, r: i32) -> bool {
        if l <= start && end <= r {
            return node.value;
        }
        let mid = start/2 + end/2;
        Self::pushDown(node);
        let mut ans = true;
        if l <= mid {
            ans &= Self::query(node.left.as_mut().unwrap(), start, mid, l, r);
        }
        if ans == false {
            return ans;
        }
        if r > mid {
            ans &= Self::query(node.right.as_mut().unwrap(), mid+1, end, l, r);
        }
        ans 
    }

    // dfs回溯更新父节点
    fn pushUp(node: &mut Node) {
        node.value = node.left.as_ref().unwrap().value & node.right.as_ref().unwrap().value;
    }

    // 根据lazy标记更新下层
    fn pushDown(node: &mut Node) {
        if node.left.is_none() {
            node.left = Some(Box::new(Node::new()));
        }
        if node.right.is_none() {
            node.right = Some(Box::new(Node::new()));
        }
        if node.lazy == true {
            node.left.as_mut().unwrap().value = node.value;
            node.right.as_mut().unwrap().value = node.value;
            node.left.as_mut().unwrap().lazy = node.lazy;
            node.right.as_mut().unwrap().lazy = node.lazy;
            node.lazy = false;
        }
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */
fn main() {
    println!("Hello, world!");
}

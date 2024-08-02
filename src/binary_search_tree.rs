use std::{cell::RefCell, cmp::Ordering, rc::Rc};

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}
#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Tree {
    root: Option<Rc<RefCell<Node>>>,
    cnt: i32,
}

#[allow(unused)]
impl Tree {
    fn new() -> Self {
        Self { root: None, cnt: 0 }
    }

    fn insert(&mut self, val: i32) {
        let mut pre = None;
        let mut cur = self.root.clone();
        while let Some(node) = cur.clone() {
            pre = Some(node.clone());
            match node.borrow().val.cmp(&val) {
                Ordering::Greater => cur.clone_from(&node.borrow().left),
                Ordering::Less => cur.clone_from(&node.borrow().right),
                Ordering::Equal => {
                    return;
                }
            }
        }

        let node = Rc::new(RefCell::new(Node::new(val)));

        if cur.is_none() {
            if let Some(pre) = pre.clone() {
                if val < pre.borrow().val {
                    pre.borrow_mut().left = Some(node);
                } else {
                    pre.borrow_mut().right = Some(node);
                }
            } else {
                self.root = Some(node);
            }
        }

        self.cnt += 1;
    }

    fn search(&self, val: i32) -> bool {
        let mut node = self.root.clone();
        while let Some(next) = node.clone() {
            match next.borrow().val.cmp(&val) {
                Ordering::Greater => node.clone_from(&next.borrow().left),
                Ordering::Less => node.clone_from(&next.borrow().right),
                Ordering::Equal => {
                    return true;
                }
            }
        }
        false
    }

    fn in_order(node: Option<Rc<RefCell<Node>>>, vec: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::in_order(node.borrow().left.clone(), vec);
            vec.push(node.borrow().val);
            Self::in_order(node.borrow().right.clone(), vec);
        }
    }

    fn in_order_tree(&self) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::in_order(self.root.clone(), &mut ans);
        ans
    }
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_insert_should_work() {
        let vec = vec![55, 33, 45, 100, 22, 80, 8, 130];
        let mut tree = Tree::new();
        for val in vec.into_iter() {
            tree.insert(val);
        }
        assert_eq!(tree.cnt, 8);

        let vec = tree.in_order_tree();
        assert_eq!(vec, vec![8, 22, 33, 45, 55, 80, 100, 130]);

        assert!(tree.search(80));
        assert!(!tree.search(60));
    }
}

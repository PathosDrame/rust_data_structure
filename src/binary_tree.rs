use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
    fn new(node: Option<Rc<RefCell<Node>>>) -> Self {
        if let Some(root) = node {
            Self {
                root: Some(root),
                cnt: 1,
            }
        } else {
            Self { root: None, cnt: 0 }
        }
    }

    fn insert(
        &mut self,
        parent: Option<Rc<RefCell<Node>>>,
        left: Option<Rc<RefCell<Node>>>,
        right: Option<Rc<RefCell<Node>>>,
    ) {
        let root = self.root.as_mut();
        if let (Some(_), Some(parent)) = (root, parent) {
            parent.borrow_mut().left.clone_from(&left);
            parent.borrow_mut().right.clone_from(&right);
            if left.is_some() {
                self.cnt += 1;
            }
            if right.is_some() {
                self.cnt += 1;
            }
        }
    }

    fn pre_order(node: Option<Rc<RefCell<Node>>>, vec: &mut Vec<i32>) {
        if let Some(node) = node {
            vec.push(node.borrow().val);
            Self::pre_order(node.borrow().left.clone(), vec);
            Self::pre_order(node.borrow().right.clone(), vec);
        }
    }

    fn pre_order_recur(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        Self::pre_order(self.root.clone(), &mut vec);
        vec
    }

    fn in_order(node: Option<Rc<RefCell<Node>>>, vec: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::in_order(node.borrow().left.clone(), vec);
            vec.push(node.borrow().val);
            Self::in_order(node.borrow().right.clone(), vec);
        }
    }

    fn in_order_recur(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        Self::in_order(self.root.clone(), &mut vec);
        vec
    }

    fn post_order(node: Option<Rc<RefCell<Node>>>, vec: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::post_order(node.borrow().left.clone(), vec);
            Self::post_order(node.borrow().right.clone(), vec);
            vec.push(node.borrow().val);
        }
    }

    fn post_order_recur(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        Self::post_order(self.root.clone(), &mut vec);
        vec
    }

    fn level_order_b_tree(&self) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut que = VecDeque::new();
        if let Some(node) = self.root.clone() {
            que.push_back(node);
            let len = que.len();
            while let Some(node) = que.pop_front() {
                ans.push(node.borrow().val);

                let left = node.borrow().left.clone();
                if let Some(left) = left {
                    que.push_back(left);
                }

                let right = node.borrow().right.clone();
                if let Some(right) = right {
                    que.push_back(right);
                }
            }
        }
        ans
    }

    fn pre_order_no_recur(&self) -> Vec<i32> {
        let mut ans = Vec::new();
        if let Some(root) = self.root.clone() {
            let mut vec = Vec::new();
            vec.push(root.clone());
            while let Some(node) = vec.pop() {
                ans.push(node.borrow().val);
                if let Some(right) = node.borrow().right.clone() {
                    vec.push(right);
                }
                if let Some(left) = node.borrow().left.clone() {
                    vec.push(left);
                }
            }
        }
        ans
    }

    fn post_order_no_recur(&self) -> Vec<i32> {
        let mut ans = Vec::new();
        if let Some(root) = self.root.clone() {
            let mut s = Vec::new();
            let mut f = Vec::new();
            s.push(root);
            while let Some(node) = s.pop() {
                f.push(node.clone());
                if let Some(left) = node.borrow().left.clone() {
                    s.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    s.push(right);
                }
            }
            while let Some(node) = f.pop() {
                ans.push(node.borrow().val);
            }
        }
        ans
    }
}

#[allow(unused)]
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
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        let mut tree = Tree::new(Some(node1.clone()));
        assert_eq!(
            Tree {
                root: Some(Rc::new(RefCell::new(Node {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                cnt: 1
            },
            tree
        );

        tree.insert(
            Some(node1.clone()),
            Some(node2.clone()),
            Some(node3.clone()),
        );

        assert_eq!(
            Tree {
                root: Some(Rc::new(RefCell::new(Node {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(Node {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(Node {
                        val: 3,
                        left: None,
                        right: None,
                    })))
                }))),
                cnt: 3
            },
            tree
        );

        tree.insert(
            Some(node2.clone()),
            Some(node4.clone()),
            Some(node5.clone()),
        );

        assert_eq!(
            Tree {
                root: Some(Rc::new(RefCell::new(Node {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(Node {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(Node {
                            val: 4,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(Node {
                            val: 5,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(Node {
                        val: 3,
                        left: None,
                        right: None,
                    })))
                }))),
                cnt: 5
            },
            tree
        );

        tree.insert(Some(node3.clone()), Some(node6.clone()), None);

        assert_eq!(
            Tree {
                root: Some(Rc::new(RefCell::new(Node {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(Node {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(Node {
                            val: 4,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(Node {
                            val: 5,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(Node {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(Node {
                            val: 6,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    })))
                }))),
                cnt: 6
            },
            tree
        );
    }

    #[test]
    fn pre_order_recur_should_work() {
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        let mut tree = Tree::new(Some(node1.clone()));

        tree.insert(
            Some(node1.clone()),
            Some(node2.clone()),
            Some(node3.clone()),
        );

        tree.insert(
            Some(node2.clone()),
            Some(node4.clone()),
            Some(node5.clone()),
        );

        tree.insert(Some(node3.clone()), Some(node6.clone()), None);

        let vec = tree.pre_order_recur();
        assert_eq!(vec![1, 2, 4, 5, 3, 6], vec);
    }

    #[test]
    fn in_order_recur_should_work() {
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        let mut tree = Tree::new(Some(node1.clone()));

        tree.insert(
            Some(node1.clone()),
            Some(node2.clone()),
            Some(node3.clone()),
        );

        tree.insert(
            Some(node2.clone()),
            Some(node4.clone()),
            Some(node5.clone()),
        );

        tree.insert(Some(node3.clone()), Some(node6.clone()), None);

        let vec = tree.in_order_recur();
        assert_eq!(vec![4, 2, 5, 1, 6, 3], vec);
    }

    #[test]
    fn pos_order_recur_should_work() {
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        let mut tree = Tree::new(Some(node1.clone()));

        tree.insert(
            Some(node1.clone()),
            Some(node2.clone()),
            Some(node3.clone()),
        );

        tree.insert(
            Some(node2.clone()),
            Some(node4.clone()),
            Some(node5.clone()),
        );

        tree.insert(Some(node3.clone()), Some(node6.clone()), None);

        let vec = tree.post_order_recur();
        assert_eq!(vec![4, 5, 2, 6, 3, 1], vec);
    }

    #[test]
    fn level_order_b_tree_should_work() {
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        let mut tree = Tree::new(Some(node1.clone()));

        tree.insert(
            Some(node1.clone()),
            Some(node2.clone()),
            Some(node3.clone()),
        );

        tree.insert(
            Some(node2.clone()),
            Some(node4.clone()),
            Some(node5.clone()),
        );

        tree.insert(Some(node3.clone()), Some(node6.clone()), None);

        let vec = tree.level_order_b_tree();

        assert_eq!(vec![1, 2, 3, 4, 5, 6], vec);
    }

    #[test]
    fn pre_order_no_recur_should_work() {
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        let mut tree = Tree::new(Some(node1.clone()));

        tree.insert(
            Some(node1.clone()),
            Some(node2.clone()),
            Some(node3.clone()),
        );

        tree.insert(
            Some(node2.clone()),
            Some(node4.clone()),
            Some(node5.clone()),
        );

        tree.insert(Some(node3.clone()), Some(node6.clone()), None);

        let vec = tree.pre_order_no_recur();
        assert_eq!(vec![1, 2, 4, 5, 3, 6], vec);
    }

    #[test]
    fn post_order_no_recur_should_work() {
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        let mut tree = Tree::new(Some(node1.clone()));

        tree.insert(
            Some(node1.clone()),
            Some(node2.clone()),
            Some(node3.clone()),
        );

        tree.insert(
            Some(node2.clone()),
            Some(node4.clone()),
            Some(node5.clone()),
        );

        tree.insert(Some(node3.clone()), Some(node6.clone()), None);

        let vec = tree.post_order_no_recur();
        assert_eq!(vec![4, 5, 2, 6, 3, 1], vec);
    }
}

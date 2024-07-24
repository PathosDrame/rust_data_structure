use std::{cell::RefCell, rc::Rc};

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct LinkQueue {
    len: usize,
    front: Option<Rc<RefCell<Node>>>,
    rear: Option<Rc<RefCell<Node>>>,
}

#[allow(unused)]
impl LinkQueue {
    fn new() -> Self {
        Self {
            len: 0,
            front: None,
            rear: None,
        }
    }

    fn push(&mut self, val: i32) {
        let node = Rc::new(RefCell::new(Node::new(val)));

        match self.rear.take() {
            Some(old) => old.borrow_mut().next = Some(node.clone()),
            None => self.front = Some(node.clone()),
        }

        self.rear = Some(node);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        self.front.take().map(|front| {
            if let Some(next) = front.borrow_mut().next.take() {
                self.front = Some(next);
            } else {
                self.rear.take();
            }
            Rc::try_unwrap(front).ok().unwrap().into_inner().val
        })
    }
}

impl Node {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_queue_push_should_work() {
        let mut que = LinkQueue::new();
        for i in 0..5 {
            que.push(i + 1);
        }
        assert_eq!(
            LinkQueue {
                len: 5,
                front: Some(Rc::new(RefCell::new(Node {
                    val: 1,
                    next: Some(Rc::new(RefCell::new(Node {
                        val: 2,
                        next: Some(Rc::new(RefCell::new(Node {
                            val: 3,
                            next: Some(Rc::new(RefCell::new(Node {
                                val: 4,
                                next: Some(Rc::new(RefCell::new(Node { val: 5, next: None })))
                            })))
                        })))
                    })))
                }))),
                rear: Some(Rc::new(RefCell::new(Node { val: 5, next: None })))
            },
            que
        );
    }

    #[test]
    fn link_queue_pop_should_work() {
        let mut que = LinkQueue::new();
        for i in 0..5 {
            que.push(i + 1);
        }
        for i in 0..5 {
            assert_eq!(Some(i + 1), que.pop());
        }
    }

    #[test]
    fn link_queue_pop_should_work_failed() {
        let mut que = LinkQueue::new();
        assert_eq!(None, que.pop());
    }
}

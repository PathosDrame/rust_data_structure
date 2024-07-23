#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct LinkList {
    top: Option<Box<Node>>,
    len: usize,
}

#[allow(unused)]
impl LinkList {
    fn new() -> Self {
        Self { top: None, len: 0 }
    }

    fn push(&mut self, val: i32) {
        let top = self.top.take();

        let mut node = Node::new(val);

        node.next = top;
        self.top = Some(Box::new(node));

        self.len += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        let top = self.top.take();

        if let Some(mut node) = top {
            let v = node.val;

            if let Some(n) = node.next {
                node = n;
                self.top = Some(node);
            } else {
                self.top = None;
            }

            self.len -= 1;
            return Some(v);
        }

        None
    }
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_should_work() {
        let mut stack = LinkList::new();
        for i in 1..=5 {
            stack.push(i);
        }
        assert_eq!(
            LinkList {
                top: Some(Box::new(Node {
                    val: 5,
                    next: Some(Box::new(Node {
                        val: 4,
                        next: Some(Box::new(Node {
                            val: 3,
                            next: Some(Box::new(Node {
                                val: 2,
                                next: Some(Box::new(Node { val: 1, next: None }))
                            }))
                        }))
                    }))
                })),
                len: 5,
            },
            stack
        );
    }

    #[test]
    fn link_list_pop_should_work() {
        let mut stack = LinkList::new();
        for i in 1..=5 {
            stack.push(i);
        }

        assert_eq!(Some(5), stack.pop());
        assert_eq!(4, stack.len);

        assert_eq!(Some(4), stack.pop());
        assert_eq!(3, stack.len);

        assert_eq!(Some(3), stack.pop());
        assert_eq!(2, stack.len);

        assert_eq!(Some(2), stack.pop());
        assert_eq!(1, stack.len);

        assert_eq!(Some(1), stack.pop());
        assert_eq!(0, stack.len);
    }

    #[test]
    fn link_list_pop_should_work_failed() {
        let mut stack = LinkList::new();
        assert_eq!(None, stack.pop());
    }
}

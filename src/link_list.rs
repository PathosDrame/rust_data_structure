use anyhow::{anyhow, Result};
/*
* 链式数组
*/

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub struct LinkList {
    head: Option<Box<Node>>,
    len: usize,
}

impl LinkList {
    #[allow(unused)]
    pub fn new() -> Self {
        let head = Some(Box::new(Node::new(0)));
        Self { head, len: 0 }
    }

    #[allow(unused)]
    pub fn insert(&mut self, idx: usize, val: i32) -> Result<()> {
        if idx < 1 || idx > self.len + 1 {
            println!("{}", 3);
            return Err(anyhow!("insert failed"));
        }

        let head = self.head.as_mut();

        let mut new = Node::new(val);

        if let Some(mut cur) = head {
            let mut cnt = 0;
            while cur.next.is_some() && cnt < idx - 1 {
                cur = cur.next.as_mut().unwrap();
                cnt += 1;
            }

            new.next = cur.next.take();
            cur.next = Some(Box::new(new));
        }

        self.len += 1;

        Ok(())
    }

    #[allow(unused)]
    fn delete(&mut self, val: i32) -> Result<()> {
        let head = self.head.as_mut();

        if let Some(mut cur) = head {
            while cur.next.is_some() {
                let node = cur.next.as_mut().unwrap();
                if node.val == val {
                    cur.next = node.next.take();
                } else {
                    cur = cur.next.as_mut().unwrap();
                }
            }
            self.len -= 1;
            return Ok(());
        }

        Err(anyhow!("not found val"))
    }

    #[allow(unused)]
    fn is_empty(&self) -> bool {
        self.len == 0
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
    fn link_list_insert_should_work() -> Result<()> {
        let mut list = LinkList::new();

        for i in 1..=5 {
            list.insert(i, i as i32)?;
        }

        assert_eq!(
            list,
            LinkList {
                head: Some(Box::new(Node {
                    val: 0,
                    next: Some(Box::new(Node {
                        val: 1,
                        next: Some(Box::new(Node {
                            val: 2,
                            next: Some(Box::new(Node {
                                val: 3,
                                next: Some(Box::new(Node {
                                    val: 4,
                                    next: Some(Box::new(Node { val: 5, next: None }))
                                }))
                            }))
                        }))
                    }))
                })),
                len: 5,
            }
        );

        Ok(())
    }

    #[test]
    fn link_list_delete_should_work() -> Result<()> {
        let mut list = LinkList::new();

        for i in 1..=5 {
            list.insert(i, i as i32)?;
        }

        list.delete(3)?;

        assert_eq!(
            list,
            LinkList {
                head: Some(Box::new(Node {
                    val: 0,
                    next: Some(Box::new(Node {
                        val: 1,
                        next: Some(Box::new(Node {
                            val: 2,
                            next: Some(Box::new(Node {
                                val: 4,
                                next: Some(Box::new(Node { val: 5, next: None }))
                            }))
                        }))
                    }))
                })),
                len: 4,
            }
        );
        Ok(())
    }

    #[test]
    fn link_list_is_empty_should_work() {
        let list = LinkList::new();

        assert!(list.is_empty())
    }
}

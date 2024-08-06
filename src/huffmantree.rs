#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct Node {
    weight: i32,
    parent: i32,
    l_child: i32,
    r_child: i32,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct HuffmanTree {
    vec: Vec<Node>,
    n: usize,
}

#[allow(unused)]
impl HuffmanTree {
    fn new(data: Vec<i32>) -> Self {
        let n = data.len();
        let m = n * 2 - 1;
        let mut vec = Vec::new();
        for v in data {
            vec.push(Node::new(v));
        }
        for i in n..m {
            vec.push(Node::new(0));
        }
        for i in n..m {
            let (f, l) = Self::find(&vec, i);
            vec[f].parent = i as i32;
            vec[l].parent = i as i32;
            vec[i].weight = vec[f].weight + vec[l].weight;
            vec[i].l_child = f as i32;
            vec[i].r_child = l as i32;
        }
        Self { vec, n }
    }

    fn find(vec: &[Node], n: usize) -> (usize, usize) {
        let (mut f, mut l) = (0, 0);
        for (i, _) in vec.iter().enumerate().take(n) {
            if vec[i].parent == 0 {
                f = i;
                break;
            }
        }
        for (i, _) in vec.iter().enumerate().take(n) {
            if vec[i].parent == 0 && vec[i].weight < vec[f].weight {
                f = i;
            }
        }
        for (i, _) in vec.iter().enumerate().take(n) {
            if vec[i].parent == 0 && i != f {
                l = i;
                break;
            }
        }
        for (i, _) in vec.iter().enumerate().take(n) {
            if vec[i].parent == 0 && vec[i].weight < vec[l].weight && f != i {
                l = i;
            }
        }
        (f, l)
    }

    fn create_huffmancode(&self) -> Vec<String> {
        let mut ans = Vec::new();
        for i in 0..self.n {
            let mut pos = i;
            let mut p = self.vec[i].parent;
            let mut vec = Vec::new();
            while p != 0 {
                if self.vec[p as usize].l_child == pos as i32 {
                    vec.push(b'0');
                } else {
                    vec.push(b'1');
                }
                pos = p as usize;
                p = self.vec[p as usize].parent;
            }
            vec.reverse();
            if let Ok(s) = String::from_utf8(vec) {
                ans.push(s);
            }
        }
        ans
    }
}

impl Node {
    fn new(weight: i32) -> Self {
        Self {
            weight,
            l_child: 0,
            r_child: 0,
            parent: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let data = vec![5, 29, 7, 8, 14, 23, 3, 11];
        let tree = HuffmanTree::new(data);
        assert_eq!(
            HuffmanTree {
                vec: vec![
                    Node {
                        weight: 5,
                        parent: 8,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 29,
                        parent: 13,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 7,
                        parent: 9,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 8,
                        parent: 9,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 14,
                        parent: 11,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 23,
                        parent: 12,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 3,
                        parent: 8,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 11,
                        parent: 10,
                        l_child: 0,
                        r_child: 0
                    },
                    Node {
                        weight: 8,
                        parent: 10,
                        l_child: 6,
                        r_child: 0
                    },
                    Node {
                        weight: 15,
                        parent: 11,
                        l_child: 2,
                        r_child: 3
                    },
                    Node {
                        weight: 19,
                        parent: 12,
                        l_child: 8,
                        r_child: 7
                    },
                    Node {
                        weight: 29,
                        parent: 13,
                        l_child: 4,
                        r_child: 9
                    },
                    Node {
                        weight: 42,
                        parent: 14,
                        l_child: 10,
                        r_child: 5
                    },
                    Node {
                        weight: 58,
                        parent: 14,
                        l_child: 1,
                        r_child: 11
                    },
                    Node {
                        weight: 100,
                        parent: 0,
                        l_child: 12,
                        r_child: 13
                    }
                ],
                n: 8
            },
            tree
        );
        let ans = tree.create_huffmancode();
        assert_eq!(
            vec![
                "0001".to_string(),
                "10".to_string(),
                "1110".to_string(),
                "1111".to_string(),
                "110".to_string(),
                "01".to_string(),
                "0000".to_string(),
                "001".to_string()
            ],
            ans
        );
    }
}

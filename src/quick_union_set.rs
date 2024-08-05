#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct QuickUnionSet {
    n: usize,
    parent: Vec<usize>,
    data: Vec<i32>,
    size: Vec<usize>,
}

#[allow(unused)]
impl QuickUnionSet {
    fn new(n: usize) -> Self {
        Self {
            n,
            parent: (0..n).collect(),
            data: vec![0; n],
            size: vec![1; n],
        }
    }

    fn init(&mut self, data: Vec<i32>, n: usize) {
        self.data[..n].copy_from_slice(&data[..n])
    }

    fn find_idx(&self, p: i32) -> Option<usize> {
        (0..self.n).find(|&i| self.data[i] == p)
    }

    fn find_root_idx(&self, p: i32) -> Option<usize> {
        if let Some(mut idx) = self.find_idx(p) {
            while self.parent[idx] != idx {
                idx = self.parent[idx];
            }
            return Some(idx);
        }
        None
    }

    fn find_root(&mut self, p: i32) -> Option<usize> {
        if let Some(mut p_root) = self.find_idx(p) {
            let mut vec = Vec::new();
            while self.parent[p_root] != p_root {
                vec.push(p_root);
                p_root = self.parent[p_root];
            }
            while let Some(idx) = vec.pop() {
                self.parent[idx] = p_root;
            }
            return Some(p_root);
        }
        None
    }

    fn is_same(&self, p: i32, q: i32) -> bool {
        let p_root = self.find_root_idx(p);
        let q_root = self.find_root_idx(q);
        if let (Some(p_root), Some(q_root)) = (p_root, q_root) {
            return p_root == q_root;
        }
        false
    }

    fn union(&mut self, p: i32, q: i32) {
        let p_root = self.find_root(p);
        let q_root = self.find_root(q);
        if let (Some(p_root), Some(q_root)) = (p_root, q_root) {
            if p_root != q_root {
                let ps = self.size[p_root];
                let qs = self.size[q_root];
                if ps >= qs {
                    self.parent[q_root] = p_root;
                    self.size[p_root] += qs;
                } else {
                    self.parent[p_root] = q_root;
                    self.size[q_root] += ps;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_union_set_should_work() {
        let mut qf = QuickUnionSet::new(9);
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        qf.init(data, 9);
        qf.union(3, 4);
        qf.union(8, 0);
        qf.union(2, 3);
        qf.union(5, 6);
        assert!(!qf.is_same(0, 2));
        assert!(qf.is_same(2, 4));
        qf.union(5, 1);
        qf.union(7, 3);
        qf.union(1, 6);
        qf.union(4, 8);
        assert_eq!(
            QuickUnionSet {
                n: 9,
                parent: vec![8, 5, 3, 3, 3, 5, 5, 3, 3],
                data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                size: vec![1, 1, 1, 6, 1, 3, 1, 1, 2]
            },
            qf
        );
    }
}

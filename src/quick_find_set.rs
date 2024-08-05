#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
struct QuickFindSet {
    size: usize,
    id: Vec<i32>,
    data: Vec<i32>,
}

#[allow(unused)]
impl QuickFindSet {
    fn new(size: usize) -> Self {
        Self {
            size,
            id: vec![0; size],
            data: vec![0; size],
        }
    }

    fn init(&mut self, data: Vec<i32>, n: usize) {
        for (i, v) in data.iter().enumerate().take(n) {
            self.id[i] = i as i32;
            self.data[i] = *v;
        }
    }

    fn find_id(&self, p: i32) -> Option<usize> {
        (0..self.size).find(|&i| self.data[i] == p)
    }

    fn is_same(&self, p: i32, q: i32) -> bool {
        let p_id = self.find_id(p);
        let q_id = self.find_id(q);

        if let (Some(p), Some(q)) = (p_id, q_id) {
            self.id[p] == self.id[q]
        } else {
            false
        }
    }

    fn union(&mut self, p: i32, q: i32) {
        let p_id = self.find_id(p);
        let q_id = self.find_id(q);

        if let (Some(p), Some(q)) = (p_id, q_id) {
            if p == q {
                return;
            }
            let q_id = self.id[q];
            for i in 0..self.size {
                if self.id[i] == q_id {
                    self.id[i] = self.id[p];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_find_set_should_work() {
        let mut qf = QuickFindSet::new(9);
        qf.init(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 9);
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
        assert!(qf.is_same(0, 2));
        assert!(qf.is_same(2, 4));
        assert_eq!(
            QuickFindSet {
                size: 9,
                data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                id: vec![7, 5, 7, 7, 7, 5, 5, 7, 7],
            },
            qf
        )
    }
}

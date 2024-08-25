struct MiniHeap {
    vec: Vec<i32>,
    len: usize,
    capacity: usize,
}

#[allow(unused)]
impl MiniHeap {
    fn new(cap: usize) -> Self {
        Self {
            vec: vec![0; cap + 1],
            len: 0,
            capacity: cap,
        }
    }

    fn insert(&mut self, val: i32) {
        if self.len + 1 > self.capacity {
            println!("is full");
            return;
        }
        self.len += 1;
        self.vec[self.len] = val;
        let mut idx = self.len;
        while idx > 1 && self.vec[idx] < self.vec[idx / 2] {
            self.vec.swap(idx, idx / 2);
            idx /= 2;
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.len < 1 {
            None
        } else {
            let ans = self.vec[1];
            self.vec[1] = self.vec[self.len];
            self.len -= 1;
            let mut k = 1;
            while k * 2 <= self.len {
                let mut idx = k * 2;
                if idx < self.len && self.vec[idx + 1] < self.vec[idx] {
                    idx += 1;
                }
                if self.vec[k] > self.vec[idx] {
                    self.vec.swap(k, idx);
                    k = idx;
                } else {
                    break;
                }
            }
            Some(ans)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let nums = vec![9, 3, 7, 6, 5, 1, 10, 2];
        let mut heap = MiniHeap::new(20);
        for n in nums {
            heap.insert(n);
        }
        assert_eq!(
            vec![0, 1, 2, 3, 5, 6, 7, 10, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            heap.vec
        );
        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(3), heap.pop());
        assert_eq!(Some(5), heap.pop());
        assert_eq!(Some(6), heap.pop());
        assert_eq!(Some(7), heap.pop());
        assert_eq!(Some(9), heap.pop());
        assert_eq!(Some(10), heap.pop());
        assert_eq!(None, heap.pop());
    }
}

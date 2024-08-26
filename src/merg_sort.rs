struct Arr {
    vec: Vec<i32>,
}

#[allow(unused)]
impl Arr {
    fn new(vec: Vec<i32>) -> Self {
        Self { vec }
    }

    fn merg(&mut self, start: usize, mid: usize, end: usize) {
        let f = mid - start + 1;
        let s = end - mid;
        let mut l = Vec::new();
        for i in 0..f {
            l.push(self.vec[i + start]);
        }
        let mut r = Vec::new();
        for i in 0..s {
            r.push(self.vec[i + mid + 1]);
        }
        let mut i = 0;
        let mut j = 0;
        let mut k = start;
        while i < f && j < s {
            if l[i] <= r[j] {
                self.vec[k] = l[i];
                i += 1;
            } else {
                self.vec[k] = r[j];
                j += 1;
            }
            k += 1;
        }
        while i < f {
            self.vec[k] = l[i];
            i += 1;
            k += 1;
        }
        while j < s {
            self.vec[k] = r[j];
            j += 1;
            k += 1;
        }
    }

    fn merg_loop(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let mid = (end - start) / 2 + start;
        self.merg_loop(start, mid);
        self.merg_loop(mid + 1, end);
        self.merg(start, mid, end);
    }

    fn merg_sort(&mut self) {
        self.merg_loop(0, self.vec.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let vec = vec![1, 9, 2, 7, 4, 6, 8, 3, 5];
        let mut arr = Arr::new(vec);
        arr.merg_sort();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], arr.vec);
    }
}

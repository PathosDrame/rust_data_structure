struct Arr {
    vec: Vec<i32>,
}

#[allow(unused)]
impl Arr {
    fn new(vec: Vec<i32>) -> Self {
        Self { vec }
    }

    fn v1(&mut self) {
        for i in 0..self.vec.len() {
            for j in 0..self.vec.len() - i - 1 {
                if self.vec[j] > self.vec[j + 1] {
                    self.vec.swap(j, j + 1);
                }
            }
        }
    }

    fn v2(&mut self) {
        for i in 0..self.vec.len() {
            let mut flag = true;
            for j in 0..self.vec.len() - i - 1 {
                if self.vec[j] > self.vec[j + 1] {
                    self.vec.swap(j, j + 1);
                    flag = false;
                }
            }
            if flag {
                break;
            }
        }
    }

    fn v3(&mut self) {
        let mut n = self.vec.len();
        loop {
            let mut idx = 0;
            for i in 0..n - 1 {
                if self.vec[i] > self.vec[i + 1] {
                    self.vec.swap(i, i + 1);
                    idx = i + 1;
                }
            }
            n = idx;
            if idx == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_should_work() {
        let vec = vec![1, 9, 2, 7, 4, 6, 8, 3, 5];
        let mut arr = Arr::new(vec);
        arr.v1();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], arr.vec);
    }

    #[test]
    fn v2_should_work() {
        let vec = vec![1, 9, 2, 7, 4, 6, 8, 3, 5];
        let mut arr = Arr::new(vec);
        arr.v2();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], arr.vec);
    }

    #[test]
    fn v3_should_work() {
        let vec = vec![1, 9, 2, 7, 4, 6, 8, 3, 5];
        let mut arr = Arr::new(vec);
        arr.v3();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], arr.vec);
    }
}

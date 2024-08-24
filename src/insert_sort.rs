struct Arr {
    vec: Vec<i32>,
}

#[allow(unused)]
impl Arr {
    fn new(vec: Vec<i32>) -> Self {
        Self { vec }
    }

    fn insert(&mut self) {
        for i in 1..self.vec.len() {
            let val = self.vec[i];
            let mut j = i;
            while j > 0 && self.vec[j - 1] > val {
                self.vec.swap(j, j - 1);
                j -= 1;
            }
            self.vec[j] = val;
        }
    }

    fn shell(&mut self) {
        let mut gap = self.vec.len() / 2;
        while gap > 0 {
            for i in gap..self.vec.len() {
                let val = self.vec[i];
                let mut j = i;
                while j >= gap && self.vec[j - gap] > val {
                    self.vec.swap(j, j - 1);
                    j -= 1;
                }
                self.vec[j] = val;
            }
            gap /= 2;
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
        arr.insert();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], arr.vec);
    }

    #[test]
    fn shell_should_work() {
        let vec = vec![1, 9, 2, 7, 4, 6, 8, 3, 5];
        let mut arr = Arr::new(vec);
        arr.shell();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], arr.vec);
    }
}

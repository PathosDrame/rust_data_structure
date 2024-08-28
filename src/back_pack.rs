use std::vec;

struct Arr {
    back: Vec<i32>,
    val: Vec<i32>,
    cap: i32,
}

#[allow(unused)]
impl Arr {
    fn new(back: Vec<i32>, val: Vec<i32>, cap: i32) -> Self {
        Self { back, val, cap }
    }

    fn best_value_01(&self, idx: usize, cap: i32) -> i32 {
        if cap <= 0 || idx == 0 {
            return 0;
        }
        let mut res = self.best_value_01(idx - 1, cap);
        if cap >= self.back[idx] {
            res = std::cmp::max(
                res,
                self.val[idx] + self.best_value_01(idx - 1, cap - self.back[idx]),
            );
        }
        res
    }

    fn knapsack01(&self) -> i32 {
        self.best_value_01(self.back.len() - 1, self.cap)
    }

    fn best_value_02(&self, idx: usize, cap: i32, vec: &mut Vec<Vec<i32>>) -> i32 {
        if idx == 0 || cap <= 0 {
            return 0;
        }
        let mut res = self.best_value_02(idx - 1, cap, vec);
        if vec[idx][cap as usize] != i32::MAX {
            return vec[idx][cap as usize];
        } else {
            res = std::cmp::max(
                res,
                self.val[idx] + self.best_value_02(idx - 1, cap - self.back[idx], vec),
            );
        }
        res
    }

    fn knapsack02(&self) -> i32 {
        let mut vec = vec![vec![i32::MAX; self.cap as usize + 1]; self.back.len()];
        self.best_value_02(self.back.len() - 1, self.cap, &mut vec)
    }

    fn knapsack_dp(&self) -> i32 {
        let c = self.cap as usize;
        let n = self.back.len();
        let mut vec = vec![vec![i32::MAX; c + 1]; n];
        for i in 0..=c {
            vec[0][i] = if self.back[0] <= i as i32 {
                i as i32
            } else {
                0
            };
        }
        for i in 1..n {
            for j in 0..=c {
                vec[i][j] = vec[i - 1][j];
                if self.back[i] <= j as i32 {
                    vec[i][j] =
                        std::cmp::max(vec[i][j], self.val[i] + vec[i - 1][j] - self.back[i]);
                }
            }
        }
        vec[n - 1][c]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_01() {
        let back = vec![1, 2, 3];
        let val = vec![6, 10, 12];
        let arr = Arr::new(back, val, 5);
        assert_eq!(22, arr.knapsack01());
    }

    #[test]
    fn should_work_02() {
        let back = vec![1, 2, 3];
        let val = vec![6, 10, 12];
        let arr = Arr::new(back, val, 5);
        assert_eq!(22, arr.knapsack02());
    }

    #[test]
    fn should_work_03() {
        let back = vec![1, 2, 3];
        let val = vec![6, 10, 12];
        let arr = Arr::new(back, val, 5);
        assert_eq!(22, arr.knapsack_dp());
    }
}

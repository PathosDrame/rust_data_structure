use std::cmp::Ordering;

struct Arr {
    vec: Vec<i32>,
}

#[allow(unused)]
impl Arr {
    fn new(vec: Vec<i32>) -> Self {
        Self { vec }
    }

    fn binary_search(&self, val: i32) -> Option<usize> {
        let mut l = 0;
        let mut r = self.vec.len() - 1;
        while l <= r {
            let mid = (r - l) / 2 + l;
            match val.cmp(&self.vec[mid]) {
                Ordering::Greater => l = mid + 1,
                Ordering::Less => r = mid - 1,
                Ordering::Equal => return Some(mid),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let vec = vec![0, 1, 2, 3, 4, 5];
        let arr = Arr::new(vec.clone());
        for (i, v) in vec.into_iter().enumerate() {
            assert_eq!(Some(i), arr.binary_search(v));
        }
    }
}

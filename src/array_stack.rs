#[allow(unused)]
const MAXSIZE: usize = 5;

#[allow(unused)]
struct ArrayStack {
    data: Vec<i32>,
    top: i32,
}

#[allow(unused)]
impl ArrayStack {
    fn new() -> Self {
        Self {
            data: vec![0; MAXSIZE],
            top: -1,
        }
    }

    fn push(&mut self, val: i32) -> bool {
        if self.top as usize == MAXSIZE - 1 {
            println!("OverFlow");
            return false;
        }

        self.top += 1;
        self.data[self.top as usize] = val;

        true
    }

    fn pop(&mut self) -> Option<i32> {
        if 0 > self.top {
            println!("UnderFlow");
            return None;
        }

        let v = self.data[self.top as usize];
        self.top -= 1;

        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_should_work() {
        let mut stack = ArrayStack::new();

        for i in 0..5 {
            stack.push(i + 1);
        }

        assert_eq!(vec![1, 2, 3, 4, 5], stack.data);
    }

    #[test]
    fn push_should_work_failed() {
        let mut stack = ArrayStack::new();

        for i in 0..5 {
            stack.push(i + 1);
        }

        assert!(!stack.push(100));
    }

    #[test]
    fn pop_should_work() {
        let mut stack = ArrayStack::new();

        for i in 0..5 {
            stack.push(i + 1);
        }

        assert_eq!(Some(5), stack.pop());
        assert_eq!(Some(4), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
    }

    #[test]
    fn pop_should_work_failed() {
        let mut stack = ArrayStack::new();

        assert_eq!(None, stack.pop());
    }
}

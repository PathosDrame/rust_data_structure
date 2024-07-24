#[allow(unused)]
const MAXSIZE: usize = 5;

#[allow(unused)]
struct ArrayQueue {
    data: Vec<i32>,
    front: usize,
    rear: usize,
}

#[allow(unused)]
impl ArrayQueue {
    fn new() -> Self {
        Self {
            data: vec![0; MAXSIZE],
            front: 0,
            rear: 0,
        }
    }

    fn push(&mut self, val: i32) -> bool {
        if (self.rear + 1) % MAXSIZE == self.front {
            return false;
        }

        self.rear = (self.rear + 1) % MAXSIZE;
        self.data[self.rear] = val;

        true
    }

    fn pop(&mut self) -> Option<i32> {
        if self.front == self.rear {
            return None;
        }

        self.front = (self.front + 1) % MAXSIZE;
        let v = self.data[self.front];

        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_queue_push_should_work() {
        let mut que = ArrayQueue::new();
        for i in 0..4 {
            que.push(i + 1);
        }

        assert_eq!(vec![0, 1, 2, 3, 4], que.data);
    }

    #[test]
    fn array_queue_push_should_work_failed() {
        let mut que = ArrayQueue::new();
        for i in 0..4 {
            que.push(i + 1);
        }

        assert!(!que.push(100));
    }

    #[test]
    fn array_queue_pop_should_work() {
        let mut que = ArrayQueue::new();
        for i in 0..4 {
            que.push(i + 1);
            // assert_eq!(Some(i + 1), que.pop());
        }

        for i in 0..4 {
            assert_eq!(Some(i + 1), que.pop());
        }
    }

    #[test]
    fn array_queue_pop_should_work_failed() {
        let mut que = ArrayQueue::new();
        assert_eq!(None, que.pop());
    }
}

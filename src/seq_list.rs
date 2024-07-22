use anyhow::{anyhow, Result};

/*
* 动态扩容数组
 */

#[allow(unused)]
#[derive(Debug)]
struct SeqList {
    vec: Vec<i32>,
    size: usize,
    cap: usize,
}

#[allow(unused)]
fn main() -> Result<()> {
    let mut list = SeqList::try_new(5)?;

    list.is_empty();

    for i in 1..=5 {
        list.insert(i, i as i32)?;
    }

    let v = list.get_idx(3)?;

    list.get(4);

    list.delete(4)?;
    Ok(())
}

impl SeqList {
    fn try_new(cap: usize) -> Result<Self> {
        if cap < 1 {
            return Err(anyhow!("new failed"));
        }
        Ok(Self {
            vec: vec![0; cap],
            size: 0,
            cap,
        })
    }

    fn insert(&mut self, idx: usize, val: i32) -> Result<()> {
        if idx < 1 || idx > self.size + 1 {
            return Err(anyhow!("insert failed "));
        }

        if self.size + 1 > self.cap {
            println!("enlarger!");
            let mut vec = vec![0; self.cap * 2];
            vec[..self.size].copy_from_slice(&self.vec[..self.size]);
            self.cap *= 2;
            self.vec = vec;
            println!("{:?}", self.vec);
        }

        for i in (idx - 1..self.size).rev() {
            self.vec[i + 1] = self.vec[i];
        }

        self.vec[idx - 1] = val;
        self.size += 1;

        Ok(())
    }

    fn delete(&mut self, idx: usize) -> Result<()> {
        if idx < 1 || idx > self.size {
            return Err(anyhow!("delete failed"));
        }

        for i in idx..self.size {
            self.vec[i - 1] = self.vec[i];
        }

        self.size -= 1;

        Ok(())
    }

    fn get_idx(&self, val: i32) -> Result<usize> {
        for i in 0..self.size {
            if self.vec[i] == val {
                return Ok(i + 1);
            }
        }
        Err(anyhow!("get idx failed"))
    }

    fn get(&self, idx: usize) -> Result<i32> {
        if idx < 1 || idx > self.size {
            return Err(anyhow!("get val failed"));
        }

        Ok(self.vec[idx - 1])
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_list_insert_should_work() -> Result<()> {
        let mut list = SeqList::try_new(5)?;

        for i in 1..=5 {
            list.insert(i, i as i32)?;
        }

        assert_eq!(vec![1, 2, 3, 4, 5], list.vec);

        list.insert(3, 6)?;

        assert_eq!(vec![1, 2, 6, 3, 4, 5, 0, 0, 0, 0], list.vec);
        assert_eq!(6, list.size);
        assert_eq!(10, list.cap);

        Ok(())
    }

    #[test]
    fn seq_list_delete_should_work() -> Result<()> {
        let mut list = SeqList::try_new(5)?;

        for i in 1..=5 {
            list.insert(i, i as i32)?;
        }

        list.delete(3)?;

        assert_eq!(vec![1, 2, 4, 5, 5], list.vec);
        assert_eq!(4, list.size);

        Ok(())
    }

    #[test]
    fn seq_list_get_idx_should_work() -> Result<()> {
        let mut list = SeqList::try_new(5)?;

        for i in 1..=5 {
            list.insert(i, i as i32)?;
        }

        list.get_idx(3)?;

        assert_eq!(3, 3);

        Ok(())
    }

    #[test]
    fn seq_list_get_should_work() -> Result<()> {
        let mut list = SeqList::try_new(5)?;

        for i in 1..=5 {
            list.insert(i, i as i32)?;
        }

        let val = list.get(3)?;

        assert_eq!(3, val);

        Ok(())
    }

    #[test]
    fn seq_list_is_empty_should_work() -> Result<()> {
        let list = SeqList::try_new(5)?;
        assert!(list.is_empty());
        Ok(())
    }
}

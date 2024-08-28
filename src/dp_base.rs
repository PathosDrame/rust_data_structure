#[allow(unused)]
fn fib01(val: usize) -> usize {
    if val == 0 {
        return 0;
    }
    if val == 1 {
        return 1;
    }
    fib01(val - 1) + fib01(val - 2)
}

#[allow(unused)]
fn fib(val: usize, vec: &mut Vec<usize>) -> usize {
    if val == 0 {
        return 0;
    }
    if val == 1 {
        return 1;
    }
    if vec[val] == usize::MAX {
        vec[val] = fib(val - 1, vec) + fib(val - 2, vec);
    }
    vec[val]
}

#[allow(unused)]
fn fib02(val: usize) -> usize {
    let mut vec = vec![usize::MAX; val + 1];
    fib(val, &mut vec);
    vec[val]
}

#[allow(unused)]
fn fib03(val: usize) -> usize {
    let mut vec = vec![0; val + 1];
    vec[0] = 0;
    vec[1] = 1;
    for i in 2..=val {
        vec[i] = vec[i - 1] + vec[i - 2];
    }
    vec[val]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_1() {
        assert_eq!(55, fib01(10));
    }

    #[test]
    fn should_work_2() {
        assert_eq!(55, fib02(10));
    }

    #[test]
    fn should_work_3() {
        assert_eq!(55, fib03(10));
    }
}

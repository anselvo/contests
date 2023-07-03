pub fn fib_recursive(n: i8) -> i128 {
    if n <= 1 { return 1 }
    return fib_recursive(n - 1) + fib_recursive(n - 2)
}

pub fn fib_while(mut n: i8) -> i128 {
    let mut sum = 1;
    let mut prev_sum = 1;
    while n != 1 {
        let tmp = sum;
        sum += prev_sum;
        prev_sum = tmp;
        n -= 1;
    }
    return sum as i128;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_test_recursive_fib() {
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(2), 2);
        assert_eq!(fib_recursive(3), 3);
        assert_eq!(fib_recursive(5), 8);
        assert_eq!(fib_recursive(10), 89);
    }

    #[test]
    fn should_test_while_fib() {
        assert_eq!(fib_while(1), 1);
        assert_eq!(fib_while(2), 2);
        assert_eq!(fib_while(3), 3);
        assert_eq!(fib_while(5), 8);
        assert_eq!(fib_while(10), 89);
    }
}

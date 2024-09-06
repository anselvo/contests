pub fn is_palindrome(x: i32) -> bool {
    if x < 0 { return false; }
    let mut digits = Vec::new();
    let mut t = x;
    while t / 10 != 0 {
        digits.push(t % 10);
        t = t / 10;
    }
    digits.push(t);
    
    let mut ans = true;
    let mut l: isize = 0;
    let mut r: isize = (digits.len() - 1) as isize;
    while l <= r {
        if digits[l as usize] != digits[r as usize] {
            ans = false;
            break;
        }
        l += 1;
        r -= 1;
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(-1), false);
    }
}
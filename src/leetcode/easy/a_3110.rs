pub fn score_of_string(s: String) -> i32 {
    let chars = s.as_bytes();
    let mut count = 0;
    for i in 1..s.len() {
        let left = chars[i-1];
        let right = chars[i];
        count += left.abs_diff(right) as i32
    }
    return count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(score_of_string("hello".to_string()), 13);
        assert_eq!(score_of_string("zaz".to_string()), 50);
    }
}
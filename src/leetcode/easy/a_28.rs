pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_char: Vec<char> = haystack.chars().into_iter().collect();
    let needle_char: Vec<char> = needle.chars().into_iter().collect();
    if haystack.len() < needle_char.len() { return -1; }
    for i in 0..haystack.len() - needle_char.len() + 1 {
        if haystack_char[i] == needle_char[0] {
            let mut full = true;
            for j in 1..needle_char.len() {
                if haystack_char[i + j] != needle_char[j] {
                    full = false;
                    break;
                }
            }
            if full { return i as i32; }
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
        assert_eq!(str_str("pamtampam".to_string(), "tam".to_string()), 3);
        assert_eq!(str_str("tatamtam".to_string(), "tam".to_string()), 2);
        assert_eq!(str_str("aaa".to_string(), "aaaa".to_string()), -1);
        assert_eq!(str_str("a".to_string(), "a".to_string()), 0);
        assert_eq!(str_str("mississippi".to_string(), "issip".to_string()), 4);
        assert_eq!(str_str("mississippi".to_string(), "issipi".to_string()), -1);
    }
}
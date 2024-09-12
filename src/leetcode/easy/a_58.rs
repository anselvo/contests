pub fn length_of_last_word(s: String) -> i32 {
    let mut ans = 0;
    for c in s.chars().rev() {
        if c == ' ' && ans != 0 { return ans; }
        if c != ' ' { ans += 1; }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
        assert_eq!(length_of_last_word(" fd".to_string()), 2);
        assert_eq!(length_of_last_word(" ".to_string()), 0);
        assert_eq!(length_of_last_word("".to_string()), 0);
        assert_eq!(length_of_last_word("s  ".to_string()), 1);
    }
}
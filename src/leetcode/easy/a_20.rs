pub fn is_valid(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut dp: Vec<char> = Vec::new();

    for c in chars {
        match c {
            '(' => { dp.push(c.clone()) }
            '[' => { dp.push(c.clone()) }
            '{' => { dp.push(c.clone()) }
            ')' => {
                let char = dp.pop();
                if char == None || char.unwrap() != '(' { return false }
            }
            ']' => {
                let char = dp.pop();
                if char == None ||  char.unwrap() != '[' { return false }
            }
            '}' => {
                let char = dp.pop();
                if char == None ||  char.unwrap() != '{' { return false }
            }
            _ => {}
        }
    }

    return if dp.len() == 0 { true } else { false };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
        assert_eq!(is_valid("([])".to_string()), true);
        assert_eq!(is_valid("(".to_string()), false);
        assert_eq!(is_valid("([)]".to_string()), false);
        assert_eq!(is_valid(")(".to_string()), false);
    }
}
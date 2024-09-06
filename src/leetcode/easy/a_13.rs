use std::collections::HashMap;


pub fn roman_to_int(s: String) -> i32 {
    let digits: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut ans = 0;
    let chars: Vec<char> = s.chars().collect();
    
    for i in 0..chars.len() - 1 {
        let cur = digits[&chars[i]];
        if cur < digits[&chars[i + 1]] {
            ans -= cur;
        } else {
            ans += cur;
        }
    }
    ans += digits[&chars[chars.len() - 1]];

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
use std::cmp::min;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 { return strs[0].clone() }
    
    let mut min_len = i8::MAX;
    let chars_list: Vec<Vec<char>> = strs.iter().map(|str| {
        let chars: Vec<char> = str.chars().collect();
        min_len = min(min_len, chars.len() as i8);
        return chars
    }).collect();

    let mut ans = String::new();
    
    for j in 0..min_len {
        let char = chars_list[0][j as usize];
        for i in 1..chars_list.len() {
            if chars_list[i][j as usize] != char { 
                return ans;
            }
        }
        ans.push(char);
    }
    
    return ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
        assert_eq!(longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
        assert_eq!(longest_common_prefix(vec!["flower".to_string()]), "flower");
        assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "".to_string()]), "");
    }
}
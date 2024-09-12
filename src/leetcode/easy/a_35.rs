pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 { return 0 }
    let (mut l, mut r) = (0, nums.len());

    while l < r {
        let half = (l + r) / 2;
        if target <= nums[half] {
            r = half;
        } else {
            l = half + 1;
        }
    }

    return l as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(vec![], 7), 0);
        assert_eq!(search_insert(vec![1], 2), 1);
        assert_eq!(search_insert(vec![1], 1), 0);
        assert_eq!(search_insert(vec![1], 0), 0);
        assert_eq!(search_insert(vec![-10, -9, -8], -7), 3);
    }
}
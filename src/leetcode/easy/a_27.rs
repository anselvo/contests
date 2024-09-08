pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[count] = nums[i];
            count += 1;
        }
    }
    return count as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec = &mut vec![3, 2, 2, 3];
        let k = remove_element(vec, 3);
        assert_eq!(k, 2);
        assert_eq!(vec[0..2], vec![2, 2]);
    }

    #[test]
    fn test2() {
        let vec = &mut vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k = remove_element(vec, 2);
        assert_eq!(k, 5);
        assert_eq!(vec[0..5], vec![0, 1, 3, 0, 4]);
    }
}
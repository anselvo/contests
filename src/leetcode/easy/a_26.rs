pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut count = 0;
    let mut v = i32::MIN;
    for i in 0..nums.len() {
        if v != nums[i] {
            nums[count] = nums[i];
            v = nums[i];
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
        let vec = &mut vec![1, 1, 2];
        let k = remove_duplicates(vec);
        assert_eq!(k, 2);
        assert_eq!(vec[0..2], vec![1, 2]);
    }

    #[test]
    fn test2() {
        let vec = &mut vec![0,0,1,1,1,2,2,3,3,4];
        let k = remove_duplicates(vec);
        assert_eq!(k, 5);
        assert_eq!(vec[0..5], vec![0,1,2,3,4]);
    }
}
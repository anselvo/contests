pub fn binary_recursive(sorted_list: Vec<usize>, el: usize) -> Option<usize> {
    let list = sorted_list.clone();
    if list.len() == 1 { return if list.last().unwrap() == &el { Some(0) } else { None } }

    let split_size = list.len() / 2;
    let left = list[..split_size].to_vec();
    let right = list[split_size..].to_vec();

    return if left.last().unwrap() >= &el {
        binary_recursive(left.clone(), el)
    } else {
        binary_recursive(right.clone(), el).map(|x| x + left.len())
    }
}

pub fn binary_while(sorted_list: Vec<usize>, el: usize) -> Option<usize> {
    let (mut l, mut r) = (0, sorted_list.len() as i32);

    while r - l > 0 {
        let m = (l + r) / 2;
        if sorted_list[m as usize] < el { l = m + 1 }
        else { r = m }
    }

    return if l < sorted_list.len() as i32 && sorted_list[l as usize] == el {
        Some(l as usize)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list() -> Vec<usize> { vec![1, 2, 3, 4, 4, 5, 5, 10, 102].clone() }

    #[test]
    fn should_test_recursive_search() {
        assert_eq!(binary_recursive(list(), 1), Some(0));
        assert_eq!(binary_recursive(list(), 4), Some(3));
        assert_eq!(binary_recursive(list(), 10), Some(7));
        assert_eq!(binary_recursive(list(), 102), Some(8));
        assert_eq!(binary_recursive(list(), 111), None);
    }

    #[test]
    fn should_test_while_search() {
        assert_eq!(binary_while(list(), 1), Some(0));
        assert_eq!(binary_while(list(), 4), Some(3));
        assert_eq!(binary_while(list(), 10), Some(7));
        assert_eq!(binary_while(list(), 102), Some(8));
        assert_eq!(binary_while(list(), 111), None);
    }
}

// O(n^2)
pub fn selection(list: Vec<usize>) -> Vec<usize> {
    let mut sorted_array = list.clone();
    for i in 0..sorted_array.len() {
        for j in i..sorted_array.len() {
            if sorted_array[j] < sorted_array[i] {
                let tmp = sorted_array[i];
                sorted_array[i] = sorted_array[j];
                sorted_array[j] = tmp;
            }
        }
    }
    return sorted_array;
}

// O(n^2)
pub fn insertion(list: Vec<usize>) -> Vec<usize> {
    let mut sorted_array = list.clone();
    for i in 1..sorted_array.len() {
        for j in (1..=i).rev() {
            if sorted_array[j] < sorted_array[j - 1] {
                let tmp = sorted_array[j];
                sorted_array[j] = sorted_array[j - 1];
                sorted_array[j - 1] = tmp;
            }
        }
    }
    return sorted_array;
}

// O(n^2)
pub fn bubble(list: Vec<usize>) -> Vec<usize> {
    let mut sorted_array = list.clone();
    for _ in 0..sorted_array.len() {
        let mut moves = 0;
        for j in 0..sorted_array.len() {
            if j + 1 < sorted_array.len() && sorted_array[j] > sorted_array[j + 1] {
                let tmp = sorted_array[j];
                sorted_array[j] = sorted_array[j + 1];
                sorted_array[j + 1] = tmp;
                moves += 1;
            }
        }
        if moves == 0 { break }
    }
    return sorted_array;
}

// O(nlogn)
pub fn merge(list: Vec<usize>) -> Vec<usize> {
    if list.len() == 1 { return list }

    let split_size = list.len() / 2;
    let left = &list[..split_size].to_vec();
    let right = &list[split_size..].to_vec();

    let sorted_left = merge(left.clone());
    let sorted_right = merge(right.clone());

    let mut merged_array = vec![];

    let (mut l, mut r) = (0, 0);
    while l + r < sorted_left.len() + sorted_right.len() {
        if l == sorted_left.len() {
            merged_array.push(sorted_right[r]);
            r += 1;
            continue;
        }
        if r == sorted_right.len() {
            merged_array.push(sorted_left[l]);
            l += 1;
            continue;
        }

        if sorted_left[l] < sorted_right[r] {
            merged_array.push(sorted_left[l]);
            l += 1;
        } else {
            merged_array.push(sorted_right[r]);
            r += 1;
        }
    }

    return merged_array;
}

// O(u8::MAX)
pub fn counting(list: Vec<usize>) -> Vec<usize> {
    const MAX_SIZE: usize = u8::MAX as usize;
    let mut count_array = [0; MAX_SIZE];
    for v in list {
        if v < MAX_SIZE { count_array[v] += 1; }
    }

    let mut sorted_array = vec![];
    for (i, v) in count_array.iter().enumerate() {
        let mut val = v.clone();
        while val > 0 {
            sorted_array.push(i.clone());
            val -= 1;
        }
    }

    return sorted_array
}


#[cfg(test)]
mod tests {
    use super::*;

    fn list() -> Vec<usize> { vec![5, 6, 2, 3, 2, 1].clone() }
    fn expected() -> Vec<usize> { vec![1, 2, 2, 3, 5, 6].clone() }

    #[test]
    fn should_test_selection_sort() {
        assert_eq!(selection(list()), expected());
    }

    #[test]
    fn should_test_insertion_sort() {
        assert_eq!(insertion(list()), expected());
    }

    #[test]
    fn should_test_bubble_sort() {
        assert_eq!(bubble(list()), expected());
    }

    #[test]
    fn should_test_merge_sort() {
        assert_eq!(merge(list()), expected());
    }

    #[test]
    fn should_test_counting_sort() {
        assert_eq!(counting(list()), expected());
    }
}

pub fn quick_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() < 2 {
        arr.to_vec()
    } else {
        let pivot = arr[0];
        let mut left = Vec::new();
        let mut right = Vec::new();
        for i in 1..arr.len() {
            if arr[i] < pivot {
                left.push(arr[i]);
            } else {
                right.push(arr[i]);
            }
        }
        quick_sort(&left)
            .into_iter()
            .chain(vec![pivot])
            .chain(quick_sort(&right))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![3, 2, 10, 7, 9, 1, 8, 5, 6, 4];
        assert_eq!(quick_sort(&arr), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn with_negative_numbers() {
        let arr = vec![10, 8, -1, 3, -25, 12, 15, 22, 47, 66, -10];
        assert_eq!(
            quick_sort(&arr),
            vec![-25, -10, -1, 3, 8, 10, 12, 15, 22, 47, 66]
        );
    }
}

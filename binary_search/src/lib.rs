pub fn binary_search(arr: &[i32], target: &i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid as usize] == *target {
            return Some(mid as usize);
        } else if arr[mid as usize] > *target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_target_is_in_the_middle() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 5;
        let result = binary_search(&arr, &target);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_when_target_is_on_the_left() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 3;
        let result = binary_search(&arr, &target);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_when_target_is_on_the_right() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 7;
        let result = binary_search(&arr, &target);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_when_target_is_not_in_the_array() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 10;
        let result = binary_search(&arr, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_with_empty_array() {
        let arr: [i32; 0] = [];
        let target = 1;
        let result = binary_search(&arr, &target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_with_one_element_array() {
        let arr = [5];
        let target = 5;
        let result = binary_search(&arr, &target);
        assert_eq!(result, Some(0));
    }
}

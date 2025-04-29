pub fn selection_sort(arr: &mut Vec<i32>) {
    let total_elements = arr.len();
    
    for i in 0..total_elements {
        let mut min_index = i;
        for j in i+1..total_elements {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = vec![5, 3, 2, 1, 4];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}

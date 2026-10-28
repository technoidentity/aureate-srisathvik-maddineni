pub fn merge_sort<T: Ord + Clone>(items: &mut [T]) {
    let len = items.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    merge_sort(&mut items[..mid]);
    merge_sort(&mut items[mid..]);

    let left = items[..mid].to_vec();
    let right = items[mid..].to_vec();

    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            items[k] = left[i].clone();
            i += 1;
        } else {
            items[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        items[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        items[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_numbers() {
        let mut numbers = vec![5, 2, 8, 1, 3];
        merge_sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn sorts_strings() {
        let mut words = vec!["orange", "apple", "banana"];
        merge_sort(&mut words);
        assert_eq!(words, vec!["apple", "banana", "orange"]);
    }

    #[test]
    fn handles_empty_input() {
        let mut numbers: Vec<i32> = vec![];
        merge_sort(&mut numbers);
        assert!(numbers.is_empty());
    }

    #[test]
    fn handles_duplicates() {
        let mut numbers = vec![4, 2, 4, 1, 2];
        merge_sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 2, 4, 4]);
    }
}

pub fn quicksort<T: Ord>(items: &mut [T]) {
    if items.len() <= 1 {
        return;
    }

    let pivot_index = partition(items);

    let (left, right) = items.split_at_mut(pivot_index);

    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord>(items: &mut [T]) -> usize {
    let last = items.len() - 1;

    let mut store_index = 0;

    for i in 0..last {
        if items[i] <= items[last] {
            items.swap(i, store_index);
            store_index += 1;
        }
    }

    items.swap(store_index, last);
    store_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_numbers() {
        let mut numbers = vec![5, 2, 8, 1, 3];
        quicksort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn sorts_strings() {
        let mut words = vec!["orange", "apple", "banana"];
        quicksort(&mut words);
        assert_eq!(words, vec!["apple", "banana", "orange"]);
    }

    #[test]
    fn handles_empty_input() {
        let mut numbers: Vec<i32> = vec![];
        quicksort(&mut numbers);
        assert!(numbers.is_empty());
    }

    #[test]
    fn handles_duplicates() {
        let mut numbers = vec![4, 2, 4, 1, 2];
        quicksort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 2, 4, 4]);
    }
}

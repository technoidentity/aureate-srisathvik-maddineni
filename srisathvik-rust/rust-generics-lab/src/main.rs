mod bst;
mod hash_table;
mod merge_sort;
mod quicksort;
fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = items.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match items[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }

    None
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    println!("{:?}", binary_search(&numbers, &30));
    println!("{:?}", binary_search(&numbers, &25));

    let letters = vec!['a', 'b', 'c', 'd'];

    println!("{:?}", binary_search(&letters, &'c'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_existing_element() {
        let numbers = vec![10, 20, 30, 40, 50];
        assert_eq!(binary_search(&numbers, &30), Some(2));
    }

    #[test]
    fn returns_none_for_missing_element() {
        let numbers = vec![10, 20, 30, 40, 50];
        assert_eq!(binary_search(&numbers, &25), None);
    }

    #[test]
    fn works_with_empty_input() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(binary_search(&numbers, &10), None);
    }

    #[test]
    fn works_with_strings() {
        let words = vec!["apple", "banana", "mango"];
        assert_eq!(binary_search(&words, &"banana"), Some(1));
    }
}

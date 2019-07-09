/// Insertion sort (swapping)
pub fn insertion_sort<T: Ord>(list: &mut [T]) {
    // visit each element in the list
    for i in 1..list.len() {
        // make a copy of the list index
        let mut j = i;
        // while the element behind you is bigger, swap places
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut list = [8, 6, 2, 1, 4];
        insertion_sort(&mut list);
        assert_eq!(list, [1, 2, 4, 6, 8]);
    }
}

/// Insertion sort (swapping)
pub fn insertion_sort<T: Ord>(list: &mut [T]) {
    // handle empty lists or lists w/ 1 item
    if list.len() <= 1 {
        return;
    }
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

/// Merge Sort
// Perform merge sort on a mutable vector of 'i32' elements.
pub fn merge_sort(list: &mut Vec<i32>) {
    let size: usize = list.len();
    let mut worker: Vec<i32> = vec![0; size];
    split(list, 0, size, &mut worker);

    // Merges 2 ordered sub-lists. Sub-lists are differentiated
    // by a start, middle & end index within a primary mutable list, 'primary'.
    // Ordering result is copied to the worker list, 'worker'
    fn merge(primary: &Vec<i32>, start: usize, mid: usize, end: usize, worker: &mut Vec<i32>) {
        let mut ptr1 = start;
        let mut ptr2 = mid;

        for i in start..end {
            // Continue to compare within each sub-list until 1 sub-list
            // is exhausted. If a sub-list is exhausted, then the remaining elements
            // in the other list are copied over, assuming they're already in order.
            if (ptr1 < mid) && (ptr2 >= end || primary[ptr1] <= primary[ptr2]) {
                worker[i] = primary[ptr1];
                ptr1 += 1;
            } else {
                worker[i] = primary[ptr2];
                ptr2 += 1;
            }
        }
    }

    // Copies all elements from a worker mutable list, 'worker', to a mutable primary
    // list, 'primary'. Index ranges are regarded to represent an ordered sublist
    // within the worker list, 'worker'.
    fn copy(primary: &mut Vec<i32>, start: usize, end: usize, worker: &Vec<i32>) {
        (start..end).for_each(|i| primary[i] = worker[i]);
    }
    // Splits a mutable list into 2 sub-lists. Split is done recursively
    // until only n sub-lists remain where n is the total number of elements in
    // the original list.
    // These sub-lists are then merged together, keeping order.
    fn split(primary: &mut Vec<i32>, start: usize, end: usize, worker: &mut Vec<i32>) {
        if end - start > 1 {
            let mid: usize = (end + start) / 2;

            split(primary, start, mid, worker); // left
            split(primary, mid, end, worker); // right
            merge(primary, start, mid, end, worker);
            copy(primary, start, end, worker);
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

    #[test]
    fn test_merge_sort() {
        let mut l = vec![3, 1, 5, 4, 2];
        merge_sort(&mut l);
        assert_eq!(l, vec![1, 2, 3, 4, 5]);
    }
}

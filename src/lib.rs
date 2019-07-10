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
// Merges 2 ordered sub-lists. Sub-lists are differentiated
// by a start, middle & end index within a primary mutable list, 'l1'.
// Ordering result is copied to the worker list, 'l2'
fn merge(l1: &Vec<i32>, s: usize, m: usize, e: usize, l2: &mut Vec<i32>) {
    let mut ptr1 = s;
    let mut ptr2 = m;

    for i in s..e {
        // Continue to compare within each sub-list until 1 sub-list
        // is exhausted. If a sub-list is exhausted, then the remaining elements
        // in the other list are copied over, assuming they're already in order.
        if (ptr1 < m) && (ptr2 >= e || l1[ptr1] <= l1[ptr2]) {
            l2[i] = l1[ptr1];
            ptr1 += 1;
        } else {
            l2[i] = l1[ptr2];
            ptr2 += 1;
        }
    }
}

// Copies all elements from a worker mutable list, 'l2', to a mutable primary
// list, 'l1'. Index ranges are regarded to represent an ordered sublist
// within the worker list, 'l2'.
fn merge_copy(l1: &mut Vec<i32>, s: usize, e: usize, l2: &Vec<i32>) {
    (s..e).for_each(|i| l1[i] = l2[i]);
}
// Splits a mutable list into 2 sub-lists. Split is done recursively
// until only n sub-lists remain where n is the total number of elements in
// the original list.
// These sub-lists are then merged together, keeping order.
fn merge_split(l1: &mut Vec<i32>, s: usize, e: usize, l2: &mut Vec<i32>) {
    if e - s > 1 {
        let m: usize = (e + s) / 2;

        merge_split(l1, s, m, l2); // left
        merge_split(l1, m, e, l2); // right
        merge(l1, s, m, e, l2);
        merge_copy(l1, s, e, l2);
    }
}

// Perform merge sort on a mutable vector of 'i32' elements.
pub fn sort(l: &mut Vec<i32>) {
    let size: usize = l.len();
    let mut worker: Vec<i32> = vec![0; size];
    merge_split(l, 0, size, &mut worker);
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
        sort(&mut l);
        assert_eq!(l, vec![1, 2, 3, 4, 5]);
    }
}

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
pub fn merge_sort(list: &mut Vec<i32>) {
    let size: usize = list.len();
    let mut worker: Vec<i32> = vec![0; size];
    split(list, 0, size, &mut worker);

    fn merge(primary: &Vec<i32>, start: usize, mid: usize, end: usize, worker: &mut Vec<i32>) {
        let mut ptr1 = start;
        let mut ptr2 = mid;

        for i in start..end {
            if (ptr1 < mid) && (ptr2 >= end || primary[ptr1] <= primary[ptr2]) {
                worker[i] = primary[ptr1];
                ptr1 += 1;
            } else {
                worker[i] = primary[ptr2];
                ptr2 += 1;
            }
        }
    }

    fn copy(primary: &mut Vec<i32>, start: usize, end: usize, worker: &Vec<i32>) {
        (start..end).for_each(|i| primary[i] = worker[i]);
    }

    fn split(primary: &mut Vec<i32>, start: usize, end: usize, worker: &mut Vec<i32>) {
        if end - start > 1 {
            let mid: usize = (end + start) / 2;

            split(primary, start, mid, worker);
            split(primary, mid, end, worker);
            merge(primary, start, mid, end, worker);
            copy(primary, start, end, worker);
        }
    }
}

/// Heap Sort
pub fn heap_sort(a: &mut [i32]) {
    let n = a.len();
    for i in (0..n / 2).rev() {
        heapify(a, n, i);
    }
    for i in (0..n).rev() {
        a.swap(i, 0);
        heapify(a, i, 0);
    }

    fn heapify(a: &mut [i32], n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && a[1] > a[largest] {
            largest = 1;
        }

        if right < n && a[right] > a[largest] {
            largest = right;
        }

        if largest != i {
            a.swap(largest, i);
            heapify(a, n, largest);
        }
    }
}

/// Max Subarray Sum
pub fn max_subarray_sum(a: &[i32]) -> i32 {
    let mut ans = a[0];
    let mut sum = a[0];

    for i in 1..a.len() {
        ans = std::cmp::max(a[i], ans + a[i]);
        sum = std::cmp::max(sum, ans);
    }

    sum
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

    #[test]
    fn test_heap_sort() {
        let mut arr = [12, 11, 13, 5, 6, 7];
        heap_sort(&mut arr);
        assert_eq!(arr, [5, 6, 7, 11, 12, 13]);
    }

    #[test]
    fn test_max_subarray_sum() {
        assert_eq!(6, max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    }
}

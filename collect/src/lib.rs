pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for (i, _) in arr.iter().enumerate() {
        for (j, _) in arr.iter().enumerate().take(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

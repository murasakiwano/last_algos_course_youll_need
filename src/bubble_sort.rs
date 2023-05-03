pub fn bubble_sort(arr: &mut [isize]) {
    let mut j = arr.len();

    while j > 0 {
        for i in 0..(j-1) {
            if arr[i] > arr[i+1] {
                let t = arr[i];
                arr[i] = arr[i+1];
                arr[i+1] = t;
            }
        }
        j -= 1;
    }
}

#[test]
fn test_bubble_sort() {
    let mut arr = [9, 3, 7, 4, 69, 420, 42];

    bubble_sort(&mut arr);
    assert_eq!(arr, [3, 4, 7, 9, 42, 69, 420]);
}

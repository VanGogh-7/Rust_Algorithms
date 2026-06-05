fn insertion_sort(arr: &mut [i32]) {
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j;

        while i > 0 && arr[i - 1] > key {
            arr[i] = arr[i - 1];
            i -= 1;
        }

        arr[i] = key;
    }
}

fn main() {
    let mut arr = vec![5, 2, 4, 6, 1, 3];

    insertion_sort(&mut arr);

    println!("{:?}", arr);
}
fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let mut index = low as isize - 1;
    let pivot = arr[high];

    for i in low..high {
        if arr[i] <= pivot {
            index += 1;
            arr.swap(index as usize, i);
        }
    }

    let ans = (index + 1) as usize;
    arr.swap(ans, high);
    ans
}

fn quick_sort_range(arr: &mut [i32], low: usize, high: usize) {
    if arr.len() <= 1 {
        return;
    }

    if low < high {
        let pivot = partition(arr, low, high);
        quick_sort_range(arr, low, pivot - 1);
        quick_sort_range(arr, pivot + 1, high);
    }
}

fn quick_sort(arr: &mut [i32]) {
    let high = arr.len() - 1;
    quick_sort_range(arr, 0, high);
}

fn main() {
    let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    quick_sort(&mut arr);
    println!("~~~ arr: {:?}", arr);
}

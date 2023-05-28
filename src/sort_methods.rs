pub fn quicksort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..len]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

pub fn execute<T: Ord>(method: &str, arr: &mut [T]) {
    match method{
        "bubblesort" => bubble_sort(arr),
        "selectionsort" => selection_sort(arr),
        "quicksort" => quicksort(arr),
        _ => panic!("Opção inválida. Tente novamente."),
    }
}
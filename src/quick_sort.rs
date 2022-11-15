fn quicksort<T: PartialEq + Clone + PartialOrd> (arr: &mut Vec<T>, low: usize, high: usize) {
    if low < high {
        let p = partition(arr, low, high);
        quicksort(arr, low, p-1);
        quicksort(arr, p+1, high);
    }
}

fn partition<T: PartialOrd + Clone> (arr: &mut Vec<T>, low: usize, high: usize) -> usize {
    let pivot = arr.get(high).unwrap().clone();
    let mut i = low;
    for j in low..high-1 {
        if arr.to_vec().get(j).unwrap() <= &pivot {
            let _ = &arr.swap(i, j);
            i += 1;
        }
    }
    let _ = &arr.swap(i, high);
    i
}

pub fn run() {
    let mut arr: Vec<i32> = vec![1, 5, 2, 7, 4, 4, 7, 3];
    let len = arr.len();
    quicksort::<i32>(&mut arr, 1, len - 1);
    for v in arr {
        println!("{}", v);
    }
}
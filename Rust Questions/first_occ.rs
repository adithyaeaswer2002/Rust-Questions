fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    let sorted_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;

    if let Some(index) = find_first_occurrence(&sorted_arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not found in the array", target);
    }
}

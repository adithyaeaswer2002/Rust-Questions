fn kth_smallest_element(nums: Vec<i32>, k: usize) -> Option<i32> {
    if k == 0 || k > nums.len() {
        return None;
    }
    
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    
    Some(sorted_nums[k - 1])
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let k = 4;
    
    if let Some(result) = kth_smallest_element(numbers, k) {
        println!("The {}-th smallest element is: {}", k, result);
    } else {
        println!("Invalid value of k or array is empty");
    }
}

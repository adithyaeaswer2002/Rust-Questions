fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_so_far = nums[0];
    let mut max_ending_here = nums[0];

    for i in 1..nums.len() {
        max_ending_here = std::cmp::max(nums[i], max_ending_here + nums[i]);
        max_so_far = std::cmp::max(max_so_far, max_ending_here);
    }

    max_so_far
}

fn main() {
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}

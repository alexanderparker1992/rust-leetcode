struct Solution;

impl Solution {
    // rotate-array
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize;
        let n = nums.len();

        let k = k % n;

        nums.reverse();

        nums[..k].reverse();
        nums[k..].reverse();
    }

    // best-time-to-buy-and-sell-stocks-II
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }
        return profit;
    }

    // contains-duplicates
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();

        for (i, num) in nums.iter().enumerate() {
            if i < nums.len() - 1 && *num == nums[i + 1] {
                return true;
            }
        }
        false
    }

    // remove-duplicates-from-sorted-arrays
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        return (i + 1) as i32;
    }

    //single-number - can't wait to look up the hyper-optimised version of this :/
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        nums.sort();
        for i in 1..(nums.len() - 1) {
            if nums[i] != nums[i - 1] && nums[i] != nums[i + 1] {
                return nums[i];
            }
        }
        if nums[0] != nums[1] {
            return nums[0];
        }
        if nums[nums.len() - 1] != nums[nums.len() - 2] {
            return nums[nums.len() - 1];
        }
        return nums[0];
    }
}

fn main() {
    let leetcode_problem = String::from("single-number");

    match leetcode_problem.as_str() {
        "contains-duplicates" => {
            let nums = [1, 2, 2, 3, 4, 5, 6, 7].to_vec();
            let sol = Solution::contains_duplicate(nums);
            println!("{}", sol);
        }

        "remove-duplicates-from-sorted-arrays" => {
            let mut nums: Vec<i32> = [1, 1, 2].to_vec();
            let result: i32 = Solution::remove_duplicates(&mut nums);
            println!("Result: {}", result);
            println!("Modified Array: {:?}", nums);
        }

        "rotate-array" => {
            let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
            Solution::rotate(&mut nums, 3);
            println!("Modified Array: {:?}", nums);
        }
        "best-time-to-buy-and-sell-stocks-II" => {
            let prices = [7, 1, 5, 3, 6, 4].to_vec();
            let result: i32 = Solution::max_profit(prices);
            println!("Result: {}", result);
        }
        "single-number" => {
            let nums = [4, 1, 2, 1, 2].to_vec();
            let result = Solution::single_number(nums);
            println!("Result: {}", result);
        }
        _ => {
            // Default case
            println!("Congratulations, you've finished leetcode");
        }
    }
}

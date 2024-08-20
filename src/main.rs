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

    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 0;
        nums1.sort();
        nums2.sort();
        let mut nums_intersect = vec![];
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                nums_intersect.push(nums1[i]);
                i += 1;
                j += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else if nums1[i] < nums2[j] {
                i += 1;
            }
        }
        return nums_intersect;
    }

    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry: bool = true;
        for i in (0..digits.len()).rev() {
            if i == digits.len() - 1 && digits[i] == 9 {
                digits[i] = 0;
                carry = true;
            } else if digits[i] == 9 && carry {
                digits[i] = 0;
            } else if carry {
                digits[i] = digits[i] + 1;
                carry = false;
            }
        }
        if carry {
            digits.insert(0, 1)
        }

        return digits;
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = nums.len();
        while i < nums.len() && i < j + 1 {
            if nums[i] == 0 {
                nums.remove(i);
                nums.push(0);
                j -= 1;
                if i > 0 {
                    i -= 1
                };
            } else {
                i += 1
            }
        }
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // rows
        for i in 0..9 {
            let mut row_set = std::collections::HashSet::new();
            for j in 0..9 {
                if board[i][j] != '.' as char {
                    if !row_set.insert(board[i][j]) {
                        return false;
                    }
                }
            }
        }

        // cols
        for j in 0..9 {
            let mut col_set = std::collections::HashSet::new();
            for i in 0..9 {
                if board[i][j] != '.' {
                    if !col_set.insert(board[i][j]) {
                        return false;
                    }
                }
            }
        }

        // sub boxes
        for chunk in 0..9 {
            let mut sub_boxes = std::collections::HashSet::new();
            let start_row = (chunk / 3) * 3;
            let start_col = (chunk % 3) * 3;
            for i in start_row..start_row + 3 {
                for j in start_col..start_col + 3 {
                    if board[i][j] != '.' {
                        if !sub_boxes.insert(board[i][j]) {
                            return false;
                        }
                    }
                }
            }
        }

        return true;
    }
}

fn main() {
    let leetcode_problem: String = String::from("valid-soduko");

    match leetcode_problem.as_str() {
        "valid-soduko" => {
            let board: Vec<Vec<char>> = vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ];
            let output: bool = Solution::is_valid_sudoku(board);
            println!("Result: {}", output);
        }
        "two-sums" => {
            let nums = [2, 7, 11, 15].to_vec();
            let target = 9;
            let result = Solution::two_sum(nums, target);
            println!("Result: {:?}", result);
        }

        "move-zeroes" => {
            let mut nums = [0, 0, 1].to_vec();
            Solution::move_zeroes(&mut nums);
            println!("Result: {:?}", nums);
        }

        "plus-one" => {
            let digits = [9, 9, 9].to_vec();
            let sol = Solution::plus_one(digits);
            println!("Result: {:?}", sol);
        }

        "intersection-of-two-arrays-II" => {
            let num2 = [4, 9, 5].to_vec();
            let num1 = [9, 4, 9, 8, 4].to_vec();
            let sol = Solution::intersect(num1, num2);
            println!("Result: {:?}", sol);
        }

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

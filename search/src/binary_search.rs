// binary_search.rs

fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    // 注意是 <= 不是 <
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;

        // 若 low + high 可能溢出，可转换为减法
        // let mid: usize = low  + ((high - low) >> 1);
        // 其中 >> 是位运算符中的右移操作，将二进制数向右移动指定的位数，相当于将一个整数除以 2 的指定次幂。
        // 等价于  let mid = left + (right - left) / 2;

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1; // num 小于中间值，省去后半部数据
        } else {
            low = mid + 1; // num 大于等于中间值，省去前半部数据
        }
    }

    found
}

// 递归实现  爆栈
fn binary_search2(nums: &[i32], num: i32) -> bool {
    // 基本情况1: 项不存在
    if 0 == nums.len() {
        return false;
    }

    let mid: usize = nums.len() >> 1;  // /2
    if num == nums[mid] {
        return true; // 基本情况2: 项存在
    } else if num < nums[mid] {
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid + 1..], num);
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

        let mut num = 3;
        let found = binary_search1(&nums, num);
        println!("{num} is in nums: {found}");

        num = 63;
        let found = binary_search1(&nums, num);
        println!("{num} is in nums: {found}");

        num = 3;
        let found = binary_search2(&nums, num);
        println!("{num} is in nums: {found}");

        num = 63;
        let found = binary_search2(&nums, num);
        println!("{num} is in nums: {found}");
    }
}

fn nums_sum1(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum1(&nums[1..])
    }
}

fn nums_sum2(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        last + nums_sum2(&nums[..nums.len() - 1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let nums = [2, 1, 7, 4, 5];
        let sum1 = nums_sum1(&nums);
        let sum2 = nums_sum2(&nums);
        println!("sum1 is {sum1}, sum2 is {sum2}");
    }
}

/*
代码中关键处是 if 和 else 语句及其形式。
if 1 == nums.len() 检查是至关重要的，因为这是函数的转折点，这里返回数字。
else 语句中调用自身，实现了类似逐层解括号并计算值 的效果，这也是被称之为递归的原因。
递归函数总会调用自身，直到到达基本情况。
*/

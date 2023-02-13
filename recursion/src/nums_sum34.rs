fn nums_sum3(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        // 使用sum来接收中间计算值
        nums_sum3(sum + nums[0], &nums[1..])
    }
}

fn nums_sum4(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        nums_sum4(sum + nums[nums.len() - 1], &nums[..nums.len() - 1])
    }
}

mod test {
    use super::*;
    #[test]
    fn it_works() {
        let nums = [2, 1, 7, 4, 5];
        let sum1 = nums_sum3(0, &nums);
        let sum2 = nums_sum4(0, &nums);
        println!("sum1 is {sum1}, sum2 is {sum2}");
    }
}
/*
子问题的结果需要直接当成参数输入下一次递归调用。
从上面的代码也可以看出，函数最后一行才是递归调用，且参数是相加的形式，较为复杂。
*/

fn sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;

    // found 表示是否找到
    //pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }
    found
}

fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;
    let mut found = false;

    // found 表示是否找到
    //pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }
    if found {
        Some(pos)
    } else {
        None
    }
}

fn order_sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;
    let mut stop = false; // 控制遇到有序数据时退出

    // found 表示是否找到
    //pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true;
        } else if num < nums[pos] {
            stop = true;
        } else {
            pos += 1;
        }
    }
    found
}
/*
    因为数据排序好了，所以数据项不在集合中时，如果小于第一项，那么比较一次就知道结果了，
    最差是比较 n 次，平均比较 n/2 次，复杂度还是 O(n)。但是这个 O(n) 比无序的查找好，
    因为大多数情况的查找符合平均情况，而平均情况的复杂度，有序数据集查找能提升一倍速度。
*/

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let num = 8;
        let nums = [9, 3, 7, 4, 1, 6, 2, 8, 5];
        let found = sequential_search(&nums, num);
        println!("{num} is in nums: {found}");

        match sequential_search_pos(&nums, num) {
            Some(pos) => println!("index of {num} is {pos}"),
            None => println!("{num} is not in nums"),
        }
    }

    #[test]
    fn it_works_order() {
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
        let num = 44;
        let found = order_sequential_search(&nums, num);
        println!("{num} is in nums: {found}");
        let num = 49;
        let found = order_sequential_search(&nums, num);
        println!("{num} is in nums: {found}");
    }
}

// 指数查找算法
fn exponential_search(arr: &[i32], target: i32) -> bool {
    let n = arr.len();
    if arr[0] == target {           // 如果目标元素就是数组的第一个元素，则返回 true
        return true;
    }
    let mut i = 1;                 // 初始步长为 1
    while i < n && arr[i] <= target {    // 按步长进行跳跃，直到找到一个包含目标元素的区间
        i *= 2;                    // 步长翻倍
    }
    let mut low = i / 2;           // 搜索区间的下界为上一次的步长
    let mut high = std::cmp::min(i, n) - 1;  // 搜索区间的上界为当前步长与数组长度的最小值减 1
    while low <= high {            // 在搜索区间中进行二分查找
        let mid = low + (high - low) / 2;
        if arr[mid] == target {    // 如果找到目标元素，则返回 true
            return true;
        } else if arr[mid] < target {   // 如果目标元素比中间元素大，则在右侧继续搜索
            low = mid + 1;
        } else {                     // 如果目标元素比中间元素小，则在左侧继续搜索
            high = mid - 1;
        }
    }
    false   // 如果搜索区间为空或者目标元素不在搜索区间中，则返回 false
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
        let target = 27;
        let found = exponential_search(&nums, target);
        println!("{target} in nums: {found}");
    }
}


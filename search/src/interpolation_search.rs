// 内插查找算法
fn interpolation_search(arr: &[i32], target: i32) -> bool {
    let n = arr.len();
    let mut low = 0;                // 搜索区间的下界
    let mut high = n - 1;           // 搜索区间的上界
    while low <= high && target >= arr[low] && target <= arr[high] {
        let pos = low + ((target - arr[low]) * (high - low) as i32 / (arr[high] - arr[low]) as i32) as usize;   // 计算目标元素在搜索区间中的比例位置
        if arr[pos] == target {      // 如果找到目标元素，则返回 true
            return true;
        } else if arr[pos] < target {   // 如果目标元素比比例位置对应的元素大，则在右侧继续搜索
            low = pos + 1;
        } else {                     // 如果目标元素比比例位置对应的元素小，则在左侧继续搜索
            high = pos - 1;
        }
    }
    false    // 如果搜索区间为空或者目标元素不在搜索区间中，则返回 false
}
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
        let target = 27;
        let found = interpolation_search(&nums, target);
        println!("{target} in nums: {found}");
    }
}

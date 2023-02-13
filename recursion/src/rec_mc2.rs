fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 全用1元纸币时的最少找零纸币数
    let mut min_cashe_num = amount;
    //最小规模问题，如果找零刚好有面值相等的硬币即返回1
    if cashes.contains(&amount) {
        // 如果有当前待找零值相同的币种
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        return min_cashes[amount as usize];
    } else {
        for c in cashes
            .iter()
            .filter(|c| *(*c) <= amount)
            .collect::<Vec<&u32>>()
        {
            // 减小问题规模，将每种情况都试完，选择找零需要的最小硬币数
            let cashe_num = 1 + rec_mc2(cashes, amount - c, min_cashes);
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
                // 找到最优解记录到表中
                min_cashes[amount as usize] = min_cashe_num;
            }
        }
    }
    min_cashe_num
}

/*
新的 rec_mc 的计算就没有那么耗时了，因为使用了变量 min_cashes 来保存中间值。
本节是讲动态规划，然而这两个程序都是递归而非动态规划，第二个程序只是在递归中保存了中间值，是一种记忆手段或者缓存。
*/

mod test {
    use std::time::{Instant, Duration};

    use super::*;

    #[test]
    fn it_works() {
        // cashes 保存各种面额的纸币
        let cashes: [u32; 5] = [1, 5, 10, 20, 50];
        let amount: u32 = 63;
        let mut min_cashes: [u32; 64] = [0; 64];
        let start = Instant::now();
        let cashe_num = rec_mc2(&cashes, amount, &mut min_cashes);
        let duration = start.elapsed();
        println!("need refund {cashe_num} cashes");
        println!("{:?}", min_cashes);
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}



fn dp_rec(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态收集从 1 到 amount 的最小找零币值数量
    // 然后从小到大凑出找零纸币数量
    for i in 1..=amount {
        let mut min_cashe_num = i;
        for c in cashes.iter().filter(|c| **c <= i).collect::<Vec<&u32>>() {
            let index = i - c;
            let cashe_num = 1 + min_cashes[index as usize];
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
            }
        }
        min_cashes[i as usize] = min_cashe_num;
    }
    // 因为收集了各个值的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}

fn dp_rec_show(cashes: &[u32], amount: u32, min_cashes: &mut [u32], used: &mut [u32]) -> u32 {
    for i in 1..=amount {
        let mut min_cashe_num = i;
        let mut used_tmp = 1;
        for c in cashes.iter().filter(|c| **c <= i).collect::<Vec<&u32>>() {
            let index = i - c;
            let cashe_num = 1 + min_cashes[index as usize];
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
                used_tmp = *c;
            }
        }
        min_cashes[i as usize] = min_cashe_num;
        used[i as usize] = used_tmp;
    }
    min_cashes[amount as usize]
}

fn print_cashes(used: &[u32], mut amount: u32){
    while amount > 0 {
        let cur = used[amount as usize];
        println!("¥{cur}");
        amount -= cur;
    }
}
mod test {
    use std::time::{Duration, Instant};

    use super::*;
    #[test]
    fn it_works() {
        // cashes 保存各种面额的纸币
        let cashes: [u32; 5] = [1, 5, 10, 20, 50];
        let amount: u32 = 63;
        let mut min_cashes: [u32; 64] = [0; 64];
        let start = Instant::now();
        let cashe_num = dp_rec(&cashes, amount, &mut min_cashes);
        let duration = start.elapsed();
        println!("need refund {cashe_num} cashes");
        println!("{:?}", min_cashes);
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }

    #[test]
    fn it_works_show() {
        // cashes 保存各种面额的纸币
        let cashes: [u32; 5] = [1, 5, 10, 20, 50];
        let amount: u32 = 63;
        let mut min_cashes: [u32; 64] = [0; 64];
        let mut used: [u32; 64] = [0; 64];
        let start = Instant::now();
        let cashe_num = dp_rec_show(&cashes, amount, &mut min_cashes, &mut used);
        let duration = start.elapsed();
        println!("need refund {cashe_num} cashes");
        println!("{:?}", used);
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        print_cashes(&used, amount);
    }
}

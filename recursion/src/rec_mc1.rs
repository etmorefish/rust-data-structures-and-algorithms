use std::time::Instant;

fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    // 全用1元纸币时的最少找零纸币数
    let mut min_cashes = amount;
    if cashes.contains(&amount) {
        return 1;
    } else {
        // 提取符合条件的币种 (找零的币值肯定要小于找零值)
        for c in cashes
            .iter()
            .filter(|&c| *c <= amount)
            .collect::<Vec<&u32>>()
        {
            // amount 减去 c， 表示使用了一张面额为 c 的纸币
            // 所以要加 1
            let num_cashes = 1 + rec_mc1(cashes, amount - c);
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }
    min_cashes
}

mod test {
    use super::*;

    #[test]
    fn it_works() {
        // cashes 保存各种面额的纸币
        let cashes = [1, 5, 10, 20, 50];
        let amount: u32 = 63;
        let start = Instant::now();
        let cashes_num = rec_mc1(&cashes, amount);
        let duration = start.elapsed();
        println!("need refund {cashes_num} cashes");
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}

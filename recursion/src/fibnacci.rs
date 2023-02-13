fn fibnacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    } else {
        fibnacci(n - 1) + fibnacci(n - 2)
    }
}
// f(n) = f(n-1) + f(n-2)

fn dp_fib(n: u32) -> u32 {
    // 只用两个位置来保存值，节约内存
    let mut dp = [1, 1];
    for i in 2..=n {
        let idx1 = (i % 2) as usize;
        let idx2 = ((i - 1) % 2) as usize;
        let idx3 = ((i - 2) % 2) as usize;
        dp[idx1] = dp[idx2] + dp[idx3];
    }
    dp[((n - 1) % 2) as usize]
}

mod test {
    use super::*;

    #[test]
    fn it_works() {
        println!("fib 10: {}", fibnacci(10));
        println!("fib 10: {}", dp_fib(10));
    }
}

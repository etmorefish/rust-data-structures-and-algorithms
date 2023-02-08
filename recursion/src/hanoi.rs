fn hanoi(height: u32, a: &str, b: &str, c: &str) {
    if 1 == height{
        println!("{a} --> {c}");
    }else {
        hanoi(height -1 , a, c, b);    // 将A柱中n-1汉罗塔移到B住
        println!("{a} --> {c}");                    // 将A中最大的盘移到C,没有移动汉罗塔，所以直接输出
        hanoi(height -1 , b, a, c);    // 将B柱中n-1汉罗塔看成整体 移到C柱
    }
}
// 虽然函数有四个参数，本质上就是从 a 移动到 c。

#[test]
fn it_works(){
    // hanoi(1, "A", "B", "C");
    // hanoi(2, "A", "B", "C");
    // hanoi(3, "A", "B", "C");
    hanoi(4, "A", "B", "C");

}
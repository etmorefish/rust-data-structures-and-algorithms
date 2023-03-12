mod binary_search;
mod sequential_search;
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    if data.contains(&3) {
        println!("Yes");
    } else {
        println!("No");
    }
}

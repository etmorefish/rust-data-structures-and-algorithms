mod deque;
mod linked_old;
mod queue;
mod stack;
fn main() {
    println!("Hello, world!");

    let s = "()[]{]}".to_string();
    let res = is_valid(s);
    println!("{}", res);
    let s = "()[]{}".to_string();
    let res = is_valid2(s);
    println!("res2: {}", res);

    println!("================================================================");
    let res = convert_to_base7(-7);
    println!("base7 : {}", res);

    // let res = eval_rpn()

    let res = last_remaining(5, 3);
    println!("last_remaining:  res: {}", res);

    println!("================================================================");
    let s = "A man, a plan, a canal: Panama".to_string();
    let s = "rustsur0".to_string();
    let res = is_palindrome(s);
    println!("is_palindrome: res: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

// 有效的括号
fn is_valid(s: String) -> bool {
    // s = "()[]{}"
    let mut stack = vec!['0'];
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return false;
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return false;
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.len() == 1
}

// 有效的括号
fn is_valid2(s: String) -> bool {
    // s = "()[]{}"
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                let res = || "([{".find(top) == ")]}".find(c);
                if !res() {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}

// 进制转换
fn convert_to_base7(mut num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    // 用栈来保存余数
    let mut stack = Vec::new();
    let sign = num.signum();
    num = num.abs();

    // 余数入栈
    while num != 0 {
        let rem = num % 7;
        stack.push(rem);
        num /= 7;
    }

    // 出栈 拼接字符串
    let mut base7 = "".to_string();
    while !stack.is_empty() {
        base7 += &stack.pop().unwrap().to_string();
    }

    // 负数，添加 “-”
    if sign < 0 {
        return "-".to_string() + &base7;
    }
    base7
}

// 后缀表达式
fn eval_rpn(tokens: Vec<String>) -> i32 {
    // 存放数字的栈
    let mut nums = Vec::new();

    for token in tokens.into_iter() {
        match token.parse() {
            Ok(n) => nums.push(n),
            Err(_) => {
                // 当匹配到运算符的时候要弹出两个数字来计算
                let r = nums.pop().unwrap();
                let l = nums.pop().unwrap();
                nums.push(do_calc(&token, l, r));
            }
        }
    }
    nums[0]
}

// 执行数学运算
fn do_calc(op: &str, a: i32, b: i32) -> i32 {
    match op {
        "*" => a * b,
        "/" => a / b,
        "+" => a + b,
        _ => a - b,
    }
}

// 剑指 Offer 62. 圆圈中最后剩下的数字
// 0、1、2、3、4这5个数字组成一个圆圈，从数字0开始每次删除第3个数字，
// 则删除的前4个数字依次是2、0、4、1，因此最后剩下的数字是3。
fn last_remaining(n: i32, m: i32) -> i32 {
    let mut q = Vec::new();
    for i in 0..n {
        q.insert(0, i);
    }
    while q.len() > 1 {
        for _ in 0..m - 1 {
            let r = q.pop().unwrap();
            println!("r = {}", r);
            q.insert(0, r);
        }
        let x = q.pop();
        println!("x: {}", x.unwrap());
    }
    q.pop().unwrap()
}

// 剑指 Offer II 018. 有效的回文
fn is_palindrome(s: String) -> bool {
    let mut d = Vec::new();

    let s = s
        .chars()
        .filter(|x| x.is_alphanumeric()) // 剔除无效字符
        .map(|x| x.to_lowercase().nth(0).unwrap()) // 转小写
        .collect::<String>();

    println!("s : {}", s);
    for c in s.chars() {
        d.push(c);
    }

    let mut flag = true;
    while d.len() > 1 && flag {
        let head = d.pop().unwrap();
        let tail = d.remove(0);

        // 比较队首和队尾字符串
        if head != tail {
            flag = false;
        }
    }
    flag
}

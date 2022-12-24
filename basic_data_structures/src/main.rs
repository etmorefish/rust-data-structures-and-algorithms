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

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works() {}
}

fn is_valid(s: String) -> bool {
    // s = "()[]{}"
    let mut stack = vec!['0'];
    for c in s.chars() {
        match c {
            '('|'['|'{' => {stack.push(c)},
            ')' => {
                if stack.pop().unwrap() != '(' {return  false;}
            },
            ']' => {
                if stack.pop().unwrap() != '[' {return  false;}
            },
            '}' => {
                if stack.pop().unwrap() != '{' {return  false;}
            },
            _ => (),
        }
    }
    stack.len() == 1

}

fn is_valid2(s: String) -> bool {
    // s = "()[]{}"
    let mut stack = Vec::new();;
    for c in s.chars() {
        match c {
            '('|'['|'{' => {stack.push(c)},
            ')'|']'|'}' => {
                if stack.is_empty(){
                    return  false;
                }
                let top = stack.pop().unwrap();
                let res = ||{"([{".find(top) == ")]}".find(c)};
                if !res(){
                    return  false;
                }
            },
            _ => (),
        }
    }
    stack.is_empty()
}

fn convert_to_base7(mut num: i32) -> String {
    if num == 0 { return "0".to_string()}
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
    if sign < 0{
        return "-".to_string() + &base7;
    }
    base7
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    // 存放数字的栈
    let mut nums = Vec::new();

    for token in tokens.into_iter(){
        match token.parse() {
            Ok(n) => {nums.push(n)},
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
fn do_calc(op: &str, a: i32, b: i32) -> i32{
    match op {
        "*" => a * b,
        "/" => a / b,
        "+" => a + b,
        _ => a -b,
    }
}
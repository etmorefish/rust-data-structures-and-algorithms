#[derive(Debug)]
struct Stack<T> {
    top: usize,   //栈顶
    data: Vec<T>, // 栈的数据
}

impl<T> Stack<T> {
    // 初始化空栈
    fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val); //数据保存在Vec尾部
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1; //栈顶减1 弹出数据
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1) //数据不能移动，返回一个引用 &
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn size(&self) -> usize {
        self.top // 直接返回栈中元素的个数==栈顶
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        println!("top: {:?}, size: {}", stack.peek().unwrap(), stack.size());
        println!("pop: {:?}, size: {}", stack.pop().unwrap(), stack.size());
        println!("is_empty: {:?}, stack: {:?}", stack.is_empty(), stack);
    }
}

// 双端队列的定义
#[derive(Debug)]
struct Deque<T> {
    cap: usize,   // capacity 容量
    data: Vec<T>, // data
}

impl<T> Deque<T> {
    fn new(size: usize) -> Self {
        Deque {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    // Vec 末尾为队首 右侧
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err(String::from("No space available"));
        }
        self.data.push(val);
        Ok(())
    }
    // Vec 首部为队尾 左侧
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err(String::from("No space available"));
        }
        self.data.insert(0, val);
        Ok(())
    }

    // 队首移除 右侧操作
    fn remove_front(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // 队尾移除 左侧操作
    fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    // 判断队列是否为空
    fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }

    // 队列长度
    fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut d = Deque::new(4);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);

        if let Err(e) = d.add_front(5) {
            println!("add_front failed: {}", e);
        }

        if let Some(data) = d.remove_rear() {
            println!("data: {data}");
        } else {
            println!("empty deque");
        }

        println!("size: {} , empty: {}", d.size(), d.is_empty());
        println!("content: {:?}", d);
    }
}

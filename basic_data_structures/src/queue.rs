// 队列的定义
#[derive(Debug)]
struct Queue<T> {
    cap: usize,   // capacity 容量
    data: Vec<T>, // data
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    // 判断是否有剩余空间，有则加入队列
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    // 数据出队
    fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
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
        let mut q = Queue::new(3);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);

        if let Err(e) = q.enqueue(4) {
            println!("Enqueue error: {e}")
        }

        if let Some(data) = q.dequeue() {
            println!("data: {data}");
        } else {
            println!("empty queue.")
        }

        println!("size: {} , empty: {}", q.size(), q.is_empty());
        println!("content: {:?}", q);
    }
}

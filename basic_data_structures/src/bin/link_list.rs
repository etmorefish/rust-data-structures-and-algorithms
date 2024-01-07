// 定义节点
#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

// 定义链表
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq> LinkedList<T> {
    // 创建一个空链表
    fn new() -> Self {
        LinkedList { head: None }
    }

    // 检查链表是否为空
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // 返回链表长度
    fn length(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    // 遍历链表并返回一个包含元素的 Vec
    fn items(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(&node.val);
            current = &node.next;
        }
        result
    }

    // 在链表头部添加元素
    fn add(&mut self, item: T) {
        let new_node = Node {
            val: item,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    // 在链表尾部添加元素
    fn append(&mut self, item: T) {
        let new_node = Node { val: item, next: None };
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(new_node));
    }

    // 在指定位置插入元素
    fn insert(&mut self, index: usize, item: T) {
        if index == 0 {
            self.add(item);
        } else {
            let mut current = &mut self.head;
            for _ in 0..index - 1 {
                if let Some(node) = current {
                    current = &mut node.next;
                } else {
                    break;
                }
            }

            if let Some(node) = current {
                let new_node = Node {
                    val: item,
                    next: node.next.take(),
                };
                node.next = Some(Box::new(new_node));
            }
        }
    }

    // 删除指定元素
    // fn remove(&mut self, item: &T) {
    //     let mut current = &mut self.head;
    //     let mut prev = None;

    //     while let Some(node) = current {
    //         // 使用 PartialEq trait 进行比较
    //         if &node.val == item {
    //             *current = node.next.take();
    //         } else {
    //             prev = Some(current);
    //             current = &mut node.next;
    //         }
    //     }
    // }

    // 查找元素是否存在
    fn find(&self, item: &T) -> bool {
        self.items().contains(&item)
    }
}

fn main() {
    let mut link_node = LinkedList::new();
    for i in 0..5 {
        link_node.append(i);
    }

    println!("{:?}", link_node.items());

    link_node.add(10);
    println!("{:?}", link_node.items());

    link_node.append(11);
    println!("{:?}", link_node.items());

    println!("{}", link_node.find(&11));

    link_node.insert(0, 21);
    println!("{:?}", link_node.items());

}

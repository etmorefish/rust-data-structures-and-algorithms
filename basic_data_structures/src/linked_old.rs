// 按照过去语言的思路
// struct Node<T> {
//     value: T,
//     next: Box<Node<T>>,
// }

// 如果这样写了就会发现一个可怕的事情，即永远都不能定义这个链表。
// 因为这个链表会不断的向您所求下一个Node是什么。正确的写法是：
#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            value: elem,
            next: None,
        }
    }

    fn set_next(&mut self, node: Self) {
        self.next = Some(Box::new(node));
    }

    fn get_last<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }

    fn push(&mut self, elem: T) {
        let new_node = Node::new(elem);
        self.get_last().set_next(new_node);
    }
}

// 如果这样写链表是没有办法写 pop 函数的。
// 因为第一个节点总是没办法 pop。
// 我们需要在第一个节点之前增加一个附设节点。称之为头结点。
// 实际上在其他语言中也是这样操作的。

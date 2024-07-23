use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/* 二叉树节点结构体 */
struct TreeNode {
    val: i32,                             // 节点值
    left: Option<Rc<RefCell<TreeNode>>>,  // 左子节点引用
    right: Option<Rc<RefCell<TreeNode>>>, // 右子节点引用
}

impl TreeNode {
    /* 构造方法 */
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }

    /* 层序遍历 */
    /// 广度优先遍历通常借助“队列”来实现。队列遵循“先进先出”的规则，
    /// 而广度优先遍历则遵循“逐层推进”的规则，两者背后的思想是一致的。
    fn level_order(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
        // 初始化队列，加入根节点
        let mut que = VecDeque::new();
        que.push_back(Rc::clone(&root));
        // 初始化一个列表，用于保存遍历序列
        let mut vec = Vec::new();

        while let Some(node) = que.pop_front() {
            // 队列出队
            vec.push(node.borrow().val); // 保存节点值
            if let Some(left) = node.borrow().left.as_ref() {
                que.push_back(Rc::clone(left)); // 左子节点入队
            }
            if let Some(right) = node.borrow().right.as_ref() {
                que.push_back(Rc::clone(right)); // 右子节点入队
            };
        }
        vec
    }

    /* 前序遍历 */
    fn pre_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        if let Some(node) = root {
            // 访问优先级：根节点 -> 左子树 -> 右子树
            result.push(node.borrow().val);
            result.append(&mut Self::pre_order(node.borrow().left.as_ref()));
            result.append(&mut Self::pre_order(node.borrow().right.as_ref()));
        }
        result
    }

    /* 中序遍历 */
    fn in_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        if let Some(node) = root {
            // 访问优先级：左子树 -> 根节点 -> 右子树
            result.append(&mut Self::in_order(node.borrow().left.as_ref()));
            result.push(node.borrow().val);
            result.append(&mut Self::in_order(node.borrow().right.as_ref()));
        }
        result
    }

    /* 后序遍历 */
    fn post_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        if let Some(node) = root {
            // 访问优先级：左子树 -> 右子树 -> 根节点
            result.append(&mut Self::post_order(node.borrow().left.as_ref()));
            result.append(&mut Self::post_order(node.borrow().right.as_ref()));
            result.push(node.borrow().val);
        }
        result
    }
}

fn main() {
    // 初始化节点
    let root = TreeNode::new(0);
    let n1 = TreeNode::new(1);
    let n2 = TreeNode::new(2);
    let n3 = TreeNode::new(3);
    let n4 = TreeNode::new(4);
    let n5 = TreeNode::new(5);
    // 构建节点之间的引用（指针）
    root.borrow_mut().left = Some(n1.clone());
    root.borrow_mut().right = Some(n2.clone());
    n1.borrow_mut().left = Some(n3);
    n1.borrow_mut().right = Some(n4);
    n2.borrow_mut().left = Some(n5.clone());

    let p = TreeNode::new(6);
    // 在 root -> n1 中间插入节点 P
    root.borrow_mut().left = Some(p.clone());
    p.borrow_mut().left = Some(n1.clone());
    println!("{:?}", TreeNode::level_order(&root));

    // 删除节点 p
    root.borrow_mut().left = Some(n1);

    println!("{:?}", TreeNode::pre_order(Some(&root)));
    println!("{:?}", TreeNode::in_order(Some(&root)));
    println!("{:?}", TreeNode::post_order(Some(&root)));
}

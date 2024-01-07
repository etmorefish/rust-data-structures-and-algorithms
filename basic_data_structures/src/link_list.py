class Node:
    def __init__(self, val) -> None:
        self.val = val  # 存放元素的地方
        self.next = None  # next是下一个节点的标识


class LinkNode:
    """单链表"""

    def __init__(self) -> None:
        self._head = None

    def is_empty(self):
        """判断链表是否为空"""
        return self._head is None

    def length(self) -> int:
        """链表长度"""
        count = 0
        cur = self._head  # 初始指针指向head
        while cur:  # 指针指向None 表示到达尾部
            count += 1
            cur = cur.next  # 指针下移
        return count

    def items(self) -> list:
        """遍历链表"""
        data_list = []
        cur = self._head
        while cur:
            data_list.append(cur.val)
            cur = cur.next
        return data_list

    def add(self, item):
        """向链表头部添加元素"""
        node = Node(item)
        node.next = self._head  # 新结点指针指向原头部结点
        self._head = node  # 头部结点指针修改为新结点

    def append(self, item):
        """尾部添加元素"""
        node = Node(item)
        if self.is_empty():  # 先判断是否为空链表
            self._head = node  # 空链表，_head 指向新结点
        else:
            cur = self._head  # 不是空链表，则找到尾部，将尾部next结点指向新结点
            while cur.next:
                cur = cur.next
            cur.next = node

    def insert(self, index, item):
        """指定位置插入元素"""
        if index <= 0:  # 指定位置在第一个元素之前，在头部插入
            self.add(item)
        elif index > (self.length() - 1):  # 指定位置超过尾部，在尾部插入
            self.append(item)
        else:
            node = Node(item)
            cur = self._head
            for i in range(index - 1):
                cur = cur.next
            node.next = cur.next
            cur.next = node

    def remove(self, item) -> None:
        """删除节点"""
        cur = self._head
        pre = None
        while cur:
            if cur.val == item:  # 如果第一个就是删除的节点
                if not pre:
                    self._head = cur.next  # 将头指针指向头节点的后一个节点
                else:
                    pre.next = cur.next  # 将删除位置前一个节点的next指向删除位置的后一个节点
                return None
            else:  # 继续按链表后移节点
                pre = cur
                cur = cur.next
        return None

    def find(self, item) -> bool:
        """查找元素是否存在"""
        return item in self.items()


if __name__ == "__main__":
    link_node = LinkNode()
    for i in range(5):
        link_node.append(i)

    print(link_node.items())

    link_node.add(10)
    print(link_node.items())

    link_node.append(11)
    print(link_node.items())

    print(link_node.find(11))

    link_node.insert(0, 21)
    print(link_node.items())

    link_node.remove(0)
    print(link_node.items())

use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn new_with_head_node(data: T) -> Self {
        LinkedList {
            head: Some(Box::new(Node { data, next: None })),
        }
    }
    // 创建一个新的结点，将新结点的 next 指向当前链表的 head 结点，然后将链表的 head 指向新结点
    pub fn insert_head(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn insert_head_with_head_node(&mut self, data: T) {

        let new_node = Node {
            data,
            next: self.head.as_mut().unwrap().next.take(),
        };
        
        self.head.as_mut().unwrap().next = Some(Box::new(new_node));

    }
    // 遍历链表，直到找到最后一个结点，然后将新结点的 next 指向 None，将最后一个结点的 next 指向新结点
    pub fn insert_tail(&mut self, data: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node { data, next: None }));
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head -> ")?;
        let mut current = &self.head;

        while let Some(node) = current {
            write!(f, "[ {} | * ] -> ", node.data)?;
            current = &node.next;
        }

        write!(f, "[ None ]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LinkedList;
    #[test]
    fn test_linked_list_creation() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.head, None);
    }

    #[test]
    fn test_linked_list_creation_with_head_node() {
        let list: LinkedList<i32> = LinkedList::new_with_head_node(10);
        assert_eq!(
            list.head,
            Some(Box::new(Node {
                data: 10,
                next: None
            }))
        );

        println!("{}", list);
    }
}

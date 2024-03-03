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
    }
}

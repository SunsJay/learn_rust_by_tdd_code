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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LinkedList;
    #[test]
    fn test_linked_list_creation() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.head, None);
    }
}


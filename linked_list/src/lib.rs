
#[derive(Debug)]
pub struct LinkedNode<T> {
    data: T,
    next: Option<Box<LinkedNode<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<LinkedNode<T>>>,
}

impl<T> LinkedNode<T> {
    pub fn new(value: T) -> Self {
        LinkedNode {
            data: value,
            next: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new(value: T) -> Self {
        LinkedList {
            head: Some(Box::new(LinkedNode::new(value))),
        }
    }

    // insert node in this linked list head
    pub fn insert_head(&mut self, value: T) {
        let new_node = LinkedNode {
            data: value,
            next: self.head.as_mut().unwrap().next.take(),
        };

        self.head.as_mut().unwrap().next = Some(Box::new(new_node));
    }
}

impl<T: std::fmt::Display> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "head -> ")?;

        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "[ {} | * ] -> ", node.data)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linked_node() {
        let node = LinkedNode::new(1);
        assert_eq!(node.data, 1);
    }

    #[test]
    fn test_linked_list() {
        let list = LinkedList::new(1);
        assert_eq!(list.head.as_ref().unwrap().data, 1);
    }
}

use linked_list::LinkedList;
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.insert_head(10);
    list.insert_head(20);
    list.insert_head(30);
    println!("{}", list);

    list.insert_head_with_head_node(10);
    list.insert_head_with_head_node(20);
    list.insert_head_with_head_node(30);
    println!("{}", list);

    let mut list: LinkedList<i32> = LinkedList::new_with_head_node(0);
    list.insert_head(10);
    list.insert_head(20);
    list.insert_head(30);
    println!("{}", list);

    list.insert_head_with_head_node(10);
    list.insert_head_with_head_node(20);
    list.insert_head_with_head_node(30);
    println!("{}", list);
}

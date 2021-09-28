#![allow(dead_code)]
use std::mem;


pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> {
    fn empty() -> Self {
        LinkedList { head: None }
    }
    
    // While the function is running, the list is shown as empty for a very short period of time.
    // However, since we hold a mutable reference, no other thread will be allowed to observe this.
    fn push(&mut self, element: T) {
        let old_head = self.head.take();
        let new_head = Some(Box::new( Node {
            value: element,
            next: old_head
        }));
        self.head = new_head
    }
    
    fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        old_head.map(|node| {
            self.head = node.next;
            node.value
        })
    }
    
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
        // match &self.head {
        //     None => None,
        //     Some(node) => Some(&node.value)
        // }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, None)
        }
    }
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;


#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::empty();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
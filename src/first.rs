// linked list has val, next
// add methods: push, pop
// implement: drop
// make generic later

use std::mem;

pub struct List {
    head: Option<Box<Node>>,
}

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, val: i32) {
        let new_node = {
            Node {
                val,
                next: mem::replace(&mut self.head, None),
            }
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

// create node class which holds the val and next

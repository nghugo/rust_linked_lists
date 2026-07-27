// linked list has val, next
// add methods: push, pop
// implement: drop
// make generic later

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, val: T) {
        let new_node = {
            Node {
                val,
                next: self.head.take(),
            }
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map (|node| {
            self.head = node.next;
            node.val}
        )
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr =self.head.take();
        while let Some(mut boxed_node) = curr {
            curr = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

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

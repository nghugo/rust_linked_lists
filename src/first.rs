pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

// --- START just for fun
trait MyAsRef<T> {
    fn as_ref2(&self) -> Option<&T>;
}

impl<T> MyAsRef<T> for Option<T> {
    fn as_ref2(&self) -> Option<&T> {
        match self {
            None => None,
            // Match ergonomics
            // 1. Rust sees self is `&Option<T>`
            // 2. Matching `Some(val)` auto-binds `val` as `&T` (borrowing)
            // 3. RHS `Some(val)` creates a BRAND NEW Option holding `&T`
            Some(val) => Some(val)
        }
    }
}
// --- END fun

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, val: T) {
        let new_node = Node {
            val,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref2().map(|node| &node.val)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
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

        assert_eq!(list.peek(), None);
        assert_eq!(list.pop(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));

        let peeked = list.peek_mut();
        peeked.map(|val| {
            *val = 9;
        });
        assert_eq!(list.peek(), Some(&9));

        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(9));
        assert_eq!(list.pop(), None);
    }
}

// STACK                                HEAP
// ┌──────────────┐                     ┌──────────────────────────┐
// │ List {       │                     │ Node 1 {                 │
// │   head: Some │ ──► Box (Pointer) ──┼─► val: 9,                │
// │ }            │                     │   next: Some ────────────┼──┐
// └──────────────┘                     └──────────────────────────┘  │
//                                                                    │
//                                      ┌──────────────────────────┐  │
//                                      │ Node 2 {                 │  │
//                                      │   val: 2,                │◄─┘
//                                      │   next: None             │
//                                      │ }                        │
//                                      └──────────────────────────┘

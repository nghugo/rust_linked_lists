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
            Some(val) => Some(val),
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
        // Previous functional style
        /*
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
         */
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.val)
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

struct IntoIter<T>(List<T>);

impl<T> List<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Previous functional style
        /*
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|next_node| &**next_node);
            &node.val
        })
         */
        let node = self.next?;
        self.next = node.next.as_deref();
        Some(&node.val)
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
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

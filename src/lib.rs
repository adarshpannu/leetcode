#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_labels)]

struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let node = Node { elem, next: self.head.take() };
        self.head = Some(Box::new(node))
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_push_pop() {
        let mut lst = crate::List::new();

        lst.push(23);
        lst.push(57);

        assert_eq!(Some(&57), lst.peek());
        assert_eq!(Some(57), lst.pop());

        assert_eq!(Some(&23), lst.peek());
        assert_eq!(Some(23), lst.pop());

        assert_eq!(None, lst.pop());
        assert_eq!(None, lst.peek());
    }
}

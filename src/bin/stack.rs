struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
        }
    }

    fn push(&mut self, item: T) {
        let node = Box::new(Node {
            item,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.item
        })
    }
}

fn main() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);

    for i in 1..3 {
        list.push(i);
    }

    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
    print_dot
}

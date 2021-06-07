use std::fmt;

struct LinkedList<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}


impl<T> LinkedList<T> where T: fmt::Display {
    fn new() -> Self {
        return LinkedList {
            root: None,
            size: 0,
        }
    }

    fn push(&mut self, v: T) {
        let newroot = Some(Box::new(Node {
            next: self.root.take(),
            value: v,
        }));
        self.root = newroot;
        self.size += 1;
    }

    fn print_all(&self) {
        let mut current = self.root.as_ref();

        while let Some(node) = current {
            println!("{}", node.value);
            current = node.next.as_ref();
        }
    }
}


fn main() {
    let mut l1 = LinkedList::new();
    l1.push("abc");
    l1.push("def");
    l1.print_all();
}

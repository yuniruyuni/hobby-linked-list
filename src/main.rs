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

    fn push_front(&mut self, v: T) {
        let newroot = Some(Box::new(Node {
            next: self.root.take(),
            value: v,
        }));
        self.root = newroot;
        self.size += 1;
    }

    fn first(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.value)
    }

    fn last(&self) -> Option<&T> {
        let mut lst = self.root.as_ref();
        let mut cur = lst;

        while let Some(node) = cur {
            lst = cur;
            cur = node.next.as_ref();
        }
        lst.map(|node| &node.value)
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
    l1.push_front("abc");
    l1.push_front("def");
    l1.print_all();

    println!("first: {}", l1.first().unwrap());
    println!("last: {}", l1.last().unwrap());

    l1.push_front("hij");
    l1.print_all();

    println!("first: {}", l1.first().unwrap());
    println!("last: {}", l1.last().unwrap());
}

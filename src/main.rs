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

    fn push_back(&mut self, v: T) {
        let mut cur = &mut self.root;

        while let Some(node) = cur {
            cur = &mut node.next;
        }

        *cur = Some(Box::new(Node {
            next: None,
            value: v,
        }));
        self.size += 1;
    }

    fn pop_front(&mut self) {
        if let Some(node) = self.root.take() {
            self.root = node.next;
        }
    }

    fn print_all(&self) {
        let mut current = self.root.as_ref();

        while let Some(node) = current {
            println!("{}", node.value);
            current = node.next.as_ref();
        }
    }

    fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            cur: self.root.as_ref(),
        }
    }
}

struct LinkedListIterator<'a, T> {
    cur: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let cur = self.cur.take();
        if let Some(node) = cur {
            self.cur = node.next.as_ref();
            Some(&node.value)
        } else {
            None
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

    l1.push_back("xyz");
    l1.print_all();

    println!("first: {}", l1.first().unwrap());
    println!("last: {}", l1.last().unwrap());

    for v in l1.iter() {
        println!("{}", v);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_front() {
        let mut l = LinkedList::new();
        l.push_front("abc");
        l.push_front("def");

        for (actual, expected) in l.iter().zip(["def", "abc"].iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn push_back() {
        let mut l = LinkedList::new();
        l.push_back("abc");
        l.push_back("def");

        for (actual, expected) in l.iter().zip(["abc", "def"].iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn push_first_some() {
        let mut l = LinkedList::new();
        l.push_back("abc");
        l.push_back("def");
        l.push_back("hij");

        assert_eq!(l.first(), Some(&"abc"));
    }

    #[test]
    fn push_first_none() {
        let l: LinkedList::<&str> = LinkedList::new();
        assert_eq!(l.first(), None);
    }

    #[test]
    fn push_last_some() {
        let mut l = LinkedList::new();
        l.push_back("abc");
        l.push_back("def");
        l.push_back("hij");

        assert_eq!(l.last(), Some(&"hij"));
    }

    #[test]
    fn push_last_none() {
        let l: LinkedList::<&str> = LinkedList::new();
        assert_eq!(l.last(), None);
    }

    #[test]
    fn pop_front() {
        let mut l = LinkedList::new();
        l.push_back("abc");
        l.push_back("def");

        l.pop_front();

        assert_eq!(l.first(), Some(&"def"));
    }
}

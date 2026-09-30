struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, value: i32) {
        let old_head = self.head.take();

        self.head = Some(Box::new(Node {
            value,
            next: old_head,
        }));
    }

    fn pop_front(&mut self) -> Option<i32> {
        let mut head = self.head.take()?;
        self.head = head.next.take();
        Some(head.value)
    }

    fn push_back(&mut self, value: i32) {
        let mut current = &mut self.head;

        loop {
            match current {
                Some(node) => {
                    current = &mut node.next;
                }
                None => {
                    *current = Some(Box::new(Node { value, next: None }));
                    break;
                }
            }
        }
    }

    fn push_back_while(&mut self, value: i32) {
        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(Box::new(Node { value, next: None }));
    }

    fn pop_back(&mut self) -> Option<i32> {
        let head = self.head.as_mut()?;
        if head.next.is_none() {
            let head = self.head.take()?;
            return Some(head.value);
        }

        let mut current = self.head.as_mut()?;

        while current.next.as_ref().unwrap().next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        let last = current.next.take()?;
        Some(last.value)
    }

    fn traverse_from_head(&self) {
        let mut temp = self.head.as_ref();

        while let Some(node) = temp {
            println!("{}", node.value);
            temp = node.next.as_ref();
        }
    }
}

fn main() {
    let mut list = LinkedList::new();

    assert_eq!(list.pop_front(), None);

    list.push_front(30);
    list.push_front(20);
    list.push_front(10);

    list.traverse_from_head();

    // assert_eq!(list.pop_front(), Some(10));
    // assert_eq!(list.pop_front(), Some(20));
    // assert_eq!(list.pop_front(), Some(30));
    // assert_eq!(list.pop_front(), None);

    list.push_back(40);
    list.push_back(50);
    list.push_back(60);

    println!("\nAfter: ");
    list.traverse_from_head();

    assert_eq!(list.pop_back(), Some(60));
    assert_eq!(list.pop_back(), Some(50));
    assert_eq!(list.pop_back(), Some(40));

    println!("\nAfter pop back operations: ");
    list.traverse_from_head();
}

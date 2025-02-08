struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add to the front of the linked list
    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // Take the current head and make it the next node
        });
        self.head = Some(new_node); // Update head to the new node
    }

    // Remove from front
    fn pop_front(&mut self) -> Option<T> {
        // Take the head node out, leaving None in its place
        self.head.take().map(|node| {
            self.head = node.next; // Update head to the next node
            node.value // Return the value of the removed node
        })
    }

    fn print(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref(); // move to the net node
        }
        println!("None");
    }
}

pub fn linked_list_fn() {
    let mut list = LinkedList::new();

    // Add elements to the list
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    // Print the list
    list.print(); // Output: 1 -> 2 -> 3 -> None

    // remove element from the front
    if let Some(value) = list.pop_front() {
        println!("Popped: {}", value);
    }

    // Print the list again
    list.print(); // Output: 2 -> 3 -> None
}

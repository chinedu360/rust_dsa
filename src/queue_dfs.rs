use std::collections::VecDeque;

struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T: std::fmt::Debug> Queue<T> {
    fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    // enqueue item (add to the back)
    fn enqueue(&mut self, item: T) {
        self.elements.push_back(item)
    }

    // dequeue items (remove from the front)
    fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    // see the first element in the queue
    fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    // check if its empty
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn print_queue(&self) {
        println!("Queue: {:?}", self.elements)
    }
}

pub fn build_queue() {
    //Queue usage
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);

    println!(
        "{:?} {:?} {}",
        queue.dequeue(),
        queue.peek_front(),
        queue.is_empty()
    );

    queue.print_queue();
}

#[derive(Debug)]
struct Stack<T> {
    element: Vec<T>,
}

impl<T: std::fmt::Debug> Stack<T> {
    fn new() -> Self {
        Stack {
            element: Vec::new(),
        }
    }

    // push item onto stack (ownership transferred to stack)
    fn push(&mut self, item: T) {
        self.element.push(item);
    }

    // pop item from stack
    fn pop(&mut self) -> Option<T> {
        self.element.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.element.last()
    }

    fn is_empty(&self) -> bool {
        self.element.is_empty()
    }

    // new methods to print all elemnets in the stack
    fn print_stack(&self) {
        println!("Stack {:?}", self.element);
    }
}

pub fn stack() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);
    println!(
        "{:?} {:?} {:?}
        ",
        stack.pop(),
        stack.peek(),
        stack.is_empty()
    );

    stack.print_stack();
}

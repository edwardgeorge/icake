pub struct Queue<A> {
    in_stack: Vec<A>,
    out_stack: Vec<A>,
}

impl<A> Queue<A> {
    fn new() -> Self {
        Queue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }
    fn enqueue(&mut self, item: A) {
        self.in_stack.push(item)
    }
    fn dequeue(&mut self) -> Option<A> {
        if self.out_stack.is_empty() {
            self.in_stack.reverse();
            std::mem::swap(&mut self.in_stack, &mut self.out_stack);
        }
        self.out_stack.pop()
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    assert_eq!(queue.dequeue(), Some(1));
    queue.enqueue(3);
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), None);
}

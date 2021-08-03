pub struct MaxStack<A> {
    stack: Vec<A>,
    max_stack: Vec<usize>,
}

impl<A> MaxStack<A> {
    pub fn new() -> Self {
        MaxStack {
            stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }
    pub fn pop(&mut self) -> Option<A> {
        self.max_stack.pop();
        self.stack.pop()
    }
    pub fn peek(&self) -> Option<&A> {
        self.stack.last()
    }
}

impl<A: Ord> MaxStack<A> {
    pub fn push(&mut self, item: A) {
        let largest_ix = match self.max_stack.last() {
            None => self.stack.len(),
            Some(last) => {
                if item >= self.stack[*last] {
                    self.stack.len()
                } else {
                    *last
                }
            }
        };
        self.max_stack.push(largest_ix);
        self.stack.push(item)
    }
    pub fn get_max(&self) -> Option<&A> {
        self.max_stack.last().map(|ix| &self.stack[*ix])
    }
}

impl<A> std::default::Default for MaxStack<A> {
    fn default() -> Self {
        MaxStack::new()
    }
}

impl<A: Ord> std::iter::FromIterator<A> for MaxStack<A> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        let mut new = MaxStack::new();
        for i in iter.into_iter() {
            new.push(i)
        }
        new
    }
}

fn main() {
    let mut stack: MaxStack<i32> = [1, 4, 3, 5, 2, 7, 1].iter().copied().collect();
    assert_eq!(stack.get_max(), Some(&7));
    stack.pop().unwrap();
    assert_eq!(stack.get_max(), Some(&7));
    stack.pop().unwrap();
    assert_eq!(stack.get_max(), Some(&5));
    stack.pop().unwrap();
    assert_eq!(stack.get_max(), Some(&5));
    stack.pop().unwrap();
    assert_eq!(stack.get_max(), Some(&4));
}

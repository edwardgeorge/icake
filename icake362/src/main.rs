#[derive(Debug, Clone)]
enum TraverseEvent<A> {
    New(A),
    VisitingLeft(A),
}

#[derive(Debug, Clone)]
pub struct Node<A> {
    pub value: A,
    pub left: Option<Box<Node<A>>>,
    pub right: Option<Box<Node<A>>>,
}

#[derive(Debug)]
pub struct NodeInOrderTraversal<'a, A> {
    events: Vec<TraverseEvent<&'a Node<A>>>,
}

impl<'a, A> From<&'a Node<A>> for NodeInOrderTraversal<'a, A> {
    fn from(value: &'a Node<A>) -> Self {
        NodeInOrderTraversal {
            events: vec![TraverseEvent::New(value)],
        }
    }
}

impl<'a, A> Iterator for NodeInOrderTraversal<'a, A> {
    type Item = &'a A;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.events.pop() {
                Some(TraverseEvent::New(e)) => {
                    if let Some(left) = e.left.as_ref() {
                        self.events.push(TraverseEvent::VisitingLeft(e));
                        self.events.push(TraverseEvent::New(left));
                    } else {
                        // short-circuit a loop by not appending a VisitingLeft event
                        if let Some(right) = e.right.as_ref() {
                            self.events.push(TraverseEvent::New(right));
                        }
                        return Some(&e.value);
                    }
                }
                Some(TraverseEvent::VisitingLeft(e)) => {
                    if let Some(right) = e.right.as_ref() {
                        self.events.push(TraverseEvent::New(right));
                    }
                    return Some(&e.value);
                }
                None => return None,
            }
        }
    }
}

fn is_binary_search_tree<A: Ord>(root: &Node<A>) -> bool {
    // we do an in-order traversal and check each item is <= the previous
    let mut iter = NodeInOrderTraversal::from(root);
    let mut prev = iter.next().unwrap(); // there must be at least one value as we started from a node
    for next in iter {
        if next < prev {
            return false;
        }
        prev = next;
    }
    // we haven't exited yet, all were in order
    true
}

fn main() {
    let in_order: Node<i32> = Node {
        value: 8,
        left: Some(Box::new(Node {
            value: 6,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            value: 10,
            left: None,
            right: None,
        })),
    };
    assert!(is_binary_search_tree(&in_order));
    assert!(is_binary_search_tree(&Node {
        value: 4,
        left: None,
        right: Some(Box::new(in_order.clone()))
    }));
    assert!(is_binary_search_tree(&Node {
        value: 12,
        left: Some(Box::new(in_order.clone())),
        right: None
    }));
    // failing trees...
    assert!(!is_binary_search_tree(&Node {
        value: 7,
        left: None,
        right: Some(Box::new(in_order.clone()))
    }));
    assert!(!is_binary_search_tree(&Node {
        value: 9,
        left: Some(Box::new(in_order.clone())),
        right: None
    }));
    assert!(!is_binary_search_tree(&Node {
        value: 9,
        left: None,
        right: Some(Box::new(in_order.clone()))
    }));
    assert!(!is_binary_search_tree(&Node {
        value: 7,
        left: Some(Box::new(in_order)),
        right: None
    }));
}

#[derive(Debug, Clone)]
pub struct Node<A> {
    pub value: A,
    pub left: Option<Box<Node<A>>>,
    pub right: Option<Box<Node<A>>>,
}

impl<A: Ord> Node<A> {
    fn insert(&mut self, new_value: A) {
        let loc = if new_value <= self.value {
            &mut self.left
        } else {
            &mut self.right
        };
        if loc.is_none() {
            let mut new = Some(Box::new(Node::from(new_value)));
            std::mem::swap(loc, &mut new);
        } else {
            loc.as_mut().unwrap().as_mut().insert(new_value);
        }
    }
}

impl<A> From<A> for Node<A> {
    fn from(value: A) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl<A: Ord> std::iter::FromIterator<A> for Node<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let mut ctx: Option<Node<A>> = None;
        for i in iter.into_iter() {
            match ctx.as_mut() {
                None => ctx = Some(Node::from(i)),
                Some(node) => {
                    node.insert(i);
                }
            }
        }
        ctx.expect("needs at least one item for root node")
    }
}

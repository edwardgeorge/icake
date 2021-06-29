#[derive(Debug)]
struct BinaryTreeNode<A> {
    value: A,
    left: Option<Box<BinaryTreeNode<A>>>,
    right: Option<Box<BinaryTreeNode<A>>>,
}

impl<A> BinaryTreeNode<A> {
    fn new(value: A) -> Self {
        BinaryTreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // methods from exercise definition
    // the name isn't great!
    fn insert_left(&mut self, value: A) {
        let val = Box::new(BinaryTreeNode::new(value));
        self.left = Some(val);
    }
    fn insert_right(&mut self, value: A) {
        let val = Box::new(BinaryTreeNode::new(value));
        self.right = Some(val);
    }
}

impl<A> BinaryTreeNode<A>
where
    A: Ord,
{
    // custom method
    fn insert(&mut self, value: A) {
        if value <= self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.insert_left(value);
            }
        } else if let Some(right) = &mut self.right {
            right.insert(value);
        } else {
            self.insert_right(value);
        }
    }
    fn from_vec(mut input: Vec<A>) -> Option<Self> {
        // make unbalanced tree from input vec
        let mut root = None;
        for i in input.drain(..) {
            match &mut root {
                None => {
                    root = Some(BinaryTreeNode::new(i));
                }
                Some(tree) => {
                    tree.insert(i);
                }
            }
        }
        root
    }
}

fn second_largest<A>(input: &BinaryTreeNode<A>) -> Option<&A> {
    let mut second = None;
    let mut cur = input;
    while let Some(next) = &cur.right {
        second = Some(&cur.value);
        cur = next.as_ref();
    }
    // if this is the last right-most node with no right child,
    // then find the last right-most child of the left node if it exists...
    if let Some(left) = &cur.left {
        cur = left.as_ref();
        while let Some(n) = &cur.right {
            cur = n.as_ref();
        }
        second = Some(&cur.value);
    }
    second
}

fn main() {
    let mut v = vec![4, 3, 5, 1, 6, 3, 7, 8, 2];
    let tree = BinaryTreeNode::from_vec(v.clone()).unwrap();
    let result = second_largest(&tree).unwrap();
    assert_eq!(*result, 7);
    // tree built from the reversed list will give an identical result
    v.reverse();
    let rtree = BinaryTreeNode::from_vec(v).unwrap();
    assert_eq!(result, second_largest(&rtree).unwrap());
    println!("second largest: {}", result);
}

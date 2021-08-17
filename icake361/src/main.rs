mod minmax;
mod tree;
use minmax::MinMax;
use tree::Node;

// create an iterator that iterates over nodes of tree, and includes depth
pub struct NodeIterator<'a, A> {
    // second item in tuple is depth of node
    nodes: Vec<(&'a Node<A>, usize)>,
}

impl<'a, A> Iterator for NodeIterator<'a, A> {
    type Item = (&'a Node<A>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, depth)) = self.nodes.pop() {
            if let Some(right) = node.right.as_ref() {
                self.nodes.push((right, depth + 1))
            }
            if let Some(left) = node.left.as_ref() {
                self.nodes.push((left, depth + 1))
            }
            Some((node, depth))
        } else {
            None
        }
    }
}

impl<'a, A> IntoIterator for &'a Node<A> {
    type Item = (&'a Node<A>, usize);
    type IntoIter = NodeIterator<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        NodeIterator {
            nodes: vec![(self, 0)],
        }
    }
}

fn is_superbalanced<A>(node: &Node<A>) -> bool {
    let mut depth = MinMax::new();
    for (node, node_depth) in node.into_iter() {
        if node.left.is_none() || node.right.is_none() {
            // is a leaf node
            depth = depth * MinMax::from(node_depth);
            if depth.difference().unwrap() > 1 {
                return false;
            }
        }
    }
    // there has to have been at least one node, so we can safely unwrap
    depth.difference().unwrap() <= 1
}

fn main() {
    let example_a: Node<i32> = [1, 2, 3, 4, 5, 6, 7].iter().cloned().collect();
    let example_b: Node<i32> = [4, 1, 7, 3, 5].iter().cloned().collect();
    let example_c: Node<i32> = [4, 1, 7, 3, 2].iter().cloned().collect();
    println!(
        "is example_a superbalanced (expecting false): {}",
        is_superbalanced(&example_a)
    );
    println!(
        "is example_b superbalanced (expecting true): {}",
        is_superbalanced(&example_b)
    );
    println!(
        "is example_c superbalanced (expecting false): {}",
        is_superbalanced(&example_c)
    );
    println!("Hello, world!");
}

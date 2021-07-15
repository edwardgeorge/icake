use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

// replicating python class closely
#[derive(PartialEq, Eq, Debug, Clone)]
struct GraphNode<Label, Colour> {
    label: Label,
    // using Vec not HashSet to simplify bounds, ie: Hash
    neighbours: Vec<Rc<RefCell<GraphNode<Label, Colour>>>>,
    colour: Option<Colour>,
}

impl<Label, Colour: Copy + Eq + Hash> GraphNode<Label, Colour> {
    fn new(label: Label) -> Self {
        GraphNode {
            label,
            neighbours: Vec::new(),
            colour: None,
        }
    }
    fn assign_colour(&mut self, colours: &[Colour]) {
        let used: HashSet<Colour> = self
            .neighbours
            .iter()
            .map(|v| v.as_ref().borrow().colour)
            .flatten()
            .collect();
        for colour in colours.iter() {
            if !used.contains(&colour) {
                self.colour = Some(*colour);
                return;
            }
        }
        panic!(
            "not enough colours for degree of graph. {} neighbours & {} colours!",
            self.neighbours.len(),
            colours.len()
        );
    }
}

fn colour_graph<Label, Colour: Copy + Eq + Hash>(
    graph: &mut [Rc<RefCell<GraphNode<Label, Colour>>>],
    colours: &[Colour],
) {
    for node in graph {
        node.as_ref().borrow_mut().assign_colour(colours);
    }
}

fn main() {
    let a: Rc<RefCell<GraphNode<&str, u8>>> = Rc::new(RefCell::new(GraphNode::new("a")));
    let b: Rc<RefCell<GraphNode<&str, u8>>> = Rc::new(RefCell::new(GraphNode::new("b")));
    let c: Rc<RefCell<GraphNode<&str, u8>>> = Rc::new(RefCell::new(GraphNode::new("c")));
    a.as_ref().borrow_mut().neighbours.push(b.clone());
    b.as_ref().borrow_mut().neighbours.push(a.clone());
    b.as_ref().borrow_mut().neighbours.push(c.clone());
    c.as_ref().borrow_mut().neighbours.push(b.clone());
    let mut graph = vec![a, b, c];
    colour_graph(&mut graph, &[1, 2, 3]);
    for i in graph.iter() {
        let node = i.as_ref().borrow();
        println!("node: {}, colour: {}", node.label, node.colour.unwrap());
    }
}

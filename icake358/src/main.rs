use std::cell::RefCell;
use std::rc::Rc;

type RefNode<A> = Rc<RefCell<Node<A>>>;

struct Node<A> {
    value: A,
    next: Option<RefNode<A>>,
}

fn contains_cycle<A: Eq>(node: &RefNode<A>) -> bool {
    let mut iter1 = node.clone();
    let mut iter2 = node.clone();
    loop {
        let node1;
        let node2;
        if let Some(next) = &iter1.as_ref().borrow().next {
            node1 = next.clone();
        } else {
            return false;
        }
        if let Some(next) = &iter2.as_ref().borrow().next {
            if let Some(next2) = &next.as_ref().borrow().next {
                node2 = next2.clone();
            } else {
                return false;
            }
        } else {
            return false;
        }
        if node1.as_ref().borrow().value == node2.as_ref().borrow().value {
            return true;
        }
        iter1 = node1;
        iter2 = node2;
    }
}

fn new_node<A>(value: A, next: Option<RefNode<A>>) -> RefNode<A> {
    Rc::new(RefCell::new(Node { value, next }))
}

fn main() {
    let node_a = new_node("a", None);
    let node_b = new_node("b", Some(node_a.clone()));
    let node_c = new_node("c", Some(node_b.clone()));
    println!("is there a cycle? {:?}", contains_cycle(&node_c));
    node_a.as_ref().borrow_mut().next = Some(node_b.clone());
    println!("is there a cycle now? {:?}", contains_cycle(&node_c));
}

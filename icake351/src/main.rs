#[derive(Debug, PartialEq)]
struct LinkedListNode<A> {
    value: A,
    // very simple linked list impl for the purposes of this exercise
    // we don't need interior mutability here or, eg, bother with arenas so will
    // keep this extra simple by just using Box for the next pointer...
    next: Option<Box<LinkedListNode<A>>>,
}

// implement an iterator that iterates over nodes in the linked list,
// essentially performing uncons and returning successive tails

impl<'a, A> IntoIterator for &'a LinkedListNode<A> {
    type Item = &'a LinkedListNode<A>;
    type IntoIter = LinkedListIterator<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator { cur: Some(self) }
    }
}

struct LinkedListIterator<'a, A> {
    cur: Option<&'a LinkedListNode<A>>,
}

impl<'a, A> Iterator for LinkedListIterator<'a, A> {
    type Item = &'a LinkedListNode<A>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.cur {
            self.cur = v.next.as_ref().map(|v| v.as_ref());
            Some(v)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn kth_node_from_end<'a, A>(
    start: &'a LinkedListNode<A>,
    k: usize,
) -> Option<&'a LinkedListNode<A>> {
    if k < 1 {
        // zeroth from end doesn't mean anything in this exercise
        return None;
    }
    // track a single node and start following pointer once we increment past 'k' nodes,
    // when we reach the end we can return the current tracked node and this will therefore
    // be the 'k'th node from end...
    let mut cur = start;
    let mut counted = 0;
    for (i, _) in start.into_iter().enumerate() {
        counted = i;
        if i > 0 && i >= k {
            // we know .unwrap() will not panic here as we are enumerating so there must be a next
            cur = cur.next.as_ref().unwrap().as_ref();
        }
    }
    if counted >= k - 1 {
        Some(cur)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let e = LinkedListNode {
            value: "Eccles",
            next: None,
        };
        let d = LinkedListNode {
            value: "Devil's Food",
            next: Some(Box::new(e)),
        };
        let c = LinkedListNode {
            value: "Cheese",
            next: Some(Box::new(d)),
        };
        let b = LinkedListNode {
            value: "Bundt",
            next: Some(Box::new(c)),
        };
        let a = LinkedListNode {
            value: "Angel Food",
            next: Some(Box::new(b)),
        };
        let r = kth_node_from_end(&a, 2);
        assert_eq!(r.is_some(), true);
        assert_eq!(r.unwrap().value, "Devil's Food");

        let r2 = kth_node_from_end(&a, 5);
        assert_eq!(r2.is_some(), true);
        assert_eq!(r2.unwrap().value, "Angel Food");

        // 6 & 7 is too many...
        let r3 = kth_node_from_end(&a, 6);
        assert_eq!(r3, None);

        let r4 = kth_node_from_end(&a, 7);
        assert_eq!(r4, None);

        // test zeroth cash
        let r5 = kth_node_from_end(&a, 0);
        assert_eq!(r5, None);
    }
}

fn main() {
    println!("use 'cargo test' to test...");
}

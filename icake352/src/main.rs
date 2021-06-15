// BONUS! version that takes a list of many sorted vecs
fn merge_lists_n<A: Ord>(mut lists: Vec<Vec<A>>) -> Vec<A> {
    let total: usize = lists.iter().map(|a| a.len()).sum();
    let mut r = Vec::with_capacity(total);
    if total == 0 {
        return r;
    }
    // use peekable drain iterators to advance each vector
    let mut v: Vec<_> = lists.iter_mut().map(|a| a.drain(..).peekable()).collect();
    while r.len() < total {
        // fold over iterators, peeking at each and accumulating the index
        // of the smallest peeked item...
        let n = v
            .iter_mut()
            .enumerate()
            .fold(None, |acc, (i, item)| match (acc, item.peek()) {
                (None, Some(v)) => Some((i, v)),
                (Some((j, v)), Some(w)) => {
                    if v <= w {
                        Some((j, v))
                    } else {
                        Some((i, w))
                    }
                }
                _ => None,
            });
        if let Some((ix, _)) = n {
            r.push(v[ix].next().unwrap());
        } else {
            panic!("Should not happen!");
        }
    }
    r
}

fn merge_lists<A: Ord>(my_list: Vec<A>, alices_list: Vec<A>) -> Vec<A> {
    merge_lists_n(vec![my_list, alices_list])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!(merge_lists_n::<u64>(vec![]).is_empty());
    }

    #[test]
    fn all_empty_vecs() {
        assert!(merge_lists_n::<u64>(vec![vec![], vec![], vec![]]).is_empty());
    }

    #[test]
    fn test_exhausting() {
        assert_eq!(
            merge_lists_n(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            merge_lists(vec![3, 4, 6, 10, 11, 15], vec![1, 5, 8, 12, 14, 19]),
            vec![1, 3, 4, 5, 6, 8, 10, 11, 12, 14, 15, 19]
        );
    }
}

fn main() {
    let my_list = vec![3, 4, 6, 10, 11, 15];
    let alices_list = vec![1, 5, 8, 12, 14, 19];
    println!("{:?}", merge_lists(my_list, alices_list));
}

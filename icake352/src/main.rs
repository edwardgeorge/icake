// BONUS! version that takes a list of many sorted vecs
#[allow(dead_code)]
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
        // this means there are 'n - 1' (where 'n' is number of 'Vec's) comparisons
        // for each item of the produced list which will mean a total number of operations
        // = O(n * m) where m == total number of items
        // whether this is more optimal than sorting (O(m log m)) depends on the shape of the input.
        // you could merge them all with a binary insertion instead
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

fn merge_lists_swap_remove<A: Ord>(mut my_list: Vec<A>, mut alices_list: Vec<A>) -> Vec<A> {
    let total = my_list.len() + alices_list.len();
    let mut r = Vec::with_capacity(total);
    let mut i = 0;
    let mut j = 0;
    while r.len() < total {
        if !my_list.is_empty() && (alices_list.is_empty() || my_list[i] <= alices_list[j]) {
            // swap_remove will move the last element to replace the element replaced apart from at the end
            // of the list where it is just removed. this means we will traverse the list half way and end up
            // with a reversed list which we will need to traverse backwards...
            r.push(my_list.swap_remove(i));
            // once we've hit the end of the list, we will just keep taking the last element of the list
            // (my_list.len() - 1) to traverse the list backwards popping the last element off...
            if !my_list.is_empty() {
                i = std::cmp::min(my_list.len() - 1, i + 1);
            }
        } else {
            r.push(alices_list.swap_remove(j));
            if !alices_list.is_empty() {
                j = std::cmp::min(alices_list.len() - 1, j + 1);
            }
        }
    }
    r
}

#[allow(dead_code)]
fn merge_lists<A: Ord>(my_list: Vec<A>, alices_list: Vec<A>) -> Vec<A> {
    //let total = my_list.len() + alices_list.len();
    //let mut r = Vec::with_capacity(total);
    //while r.len() < total {
    //    // if we want to own the items in the resulting array we have to use Vec::remove
    //    // this is NOT a O(1) operation!
    //    // if we want to use an O(1) operation then we either need to return a Vec of references
    //    // or we need to swap in some fresh items into the list (perhaps using Default?)
    //    // Vec _does_ have swap_remove which is O(1) could be used to iterate forward and then back over the list,
    //    // see merge_lists_swap_remove() implementation
    //    if !my_list.is_empty() && (alices_list.is_empty() || my_list[0] <= alices_list[0]) {
    //        r.push(my_list.remove(0));
    //    } else {
    //        r.push(alices_list.remove(0))
    //    }
    //}
    //r
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
    println!("{:?}", merge_lists_swap_remove(my_list, alices_list));
}

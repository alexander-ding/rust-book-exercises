//! In this file, you will implement two simple algorithms.
//! The goal is to familiarize you with the basics of working with references.
//!
//! Both of these problems involve the `Vec` datatype. I would take a look the `Vec` documentation:
//! https://doc.rust-lang.org/std/vec/struct.Vec.html

use std::collections::{HashMap, HashSet};

/// P1a: `insort` is a function that takes a sorted vector `v`, and inserts an element `n` into `v`
/// such that `v` remains sorted.
///
/// You may assume that `v` is already sorted, and do not need to check this fact.
///
/// Run `cargo test insort` to check your answers.

fn binary_search(v: &Vec<i32>, n: i32) -> usize {
    let mut start: usize = 0;
    let mut end: usize = v.len();

    while start < end {
        let mid = (start + end) / 2;
        let mid_value = *v.get(mid).expect("mid should never be out of bound");
        if mid_value == n {
            return mid;
        } else if mid_value < n {
            start = mid + 1
        } else {
            end = mid
        }
    }

    return start;
}

pub fn insort(v: &mut Vec<i32>, n: i32) {
    let insertion_index = binary_search(v, n);
    v.insert(insertion_index, n);
}

type Node = i32;

/// P1b: `connected` is a function that takes an edge-list representation `edges` of a *directed* graph
/// (i.e. edges has the form `[(&from, &to), ...]`) as well as a source `src` and destination `dst`.
/// `connected` returns true if there exists a path from `src` to `dst` in `edges`.
///
/// Note: in this graph representation, references to nodes are not e.g. indices into a vector, but actual
/// Rust references. You need to be very careful when comparing two nodes for equality. For example, in the program:
///   
///    let x = 1; let y = 1;
///    assert!(&x == &y)
///
/// Then this assertion passes because Rust does an implicit dereference on equality checks. You will need
/// to use the [`std::ptr::eq`](https://doc.rust-lang.org/std/ptr/fn.eq.html) function to implement `connected`.
///
/// Run `cargo test connected` to check your answers.
pub fn connected(edges: &[(&Node, &Node)], src: &Node, dst: &Node) -> bool {
    // build an outgoing neighbor edge map
    let mut outgoing_neighbors: HashMap<&Node, Vec<&Node>> = HashMap::new();
    for (from, to) in edges.iter() {
        if !outgoing_neighbors.contains_key(from) {
            outgoing_neighbors.insert(*from, Vec::new());
        }
        let neighbors = outgoing_neighbors
            .get_mut(from)
            .expect("Key should be created");
        neighbors.push(*to);
    }

    let mut frontier: Vec<&Node> = vec![src];
    let mut visited: HashSet<&Node> = HashSet::new();

    loop {
        match frontier.pop() {
            None => return false,
            Some(node) => {
                if visited.contains(node) {
                    continue;
                }
                visited.insert(node);

                let neighbors = outgoing_neighbors.get(node).expect("unknown node {*node}");
                for &neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        continue;
                    }
                    if std::ptr::eq(dst, neighbor) {
                        return true;
                    }
                    frontier.push(neighbor);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insort_test() {
        let mut v = vec![1, 5, 8];

        insort(&mut v, 0);
        assert_eq!(v, vec![0, 1, 5, 8]);

        insort(&mut v, 3);
        assert_eq!(v, vec![0, 1, 3, 5, 8]);

        insort(&mut v, 9);
        assert_eq!(v, vec![0, 1, 3, 5, 8, 9]);
    }

    #[test]
    fn connected_test() {
        let nodes = vec![1, 1, 1];
        let edges = vec![(&nodes[0], &nodes[1]), (&nodes[1], &nodes[2])];
        assert!(connected(&edges, &nodes[0], &nodes[2]));
        assert!(!connected(&edges, &nodes[2], &nodes[0]))
    }
}

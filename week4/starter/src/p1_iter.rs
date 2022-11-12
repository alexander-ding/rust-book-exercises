//! P1: Cartesian product iterator
//!
//! To get experience with traits and generics, you will implement a new kind
//! of iterator: cartesian product. The product of two iterators is the set
//! of all pairs of items from each iterator. For example:
//!
//! [1, 2] x [3, 4]  =  [(1, 3), (1, 4), (2, 3), (2, 4)]
//!
//! Your task is to design all the structs, traits, and impls that are needed
//! to make this code snippet possible:
//!
//! ```ignore
//! let h1 = hashset![1, 2];
//! let h2 = hashset![3, 4];
//! let product =
//!   h1.into_iter()
//!   .cartesian_product(h2.into_iter())
//!   .collect::<HashSet<_>>();
//! ```
//!
//! That is, there should be a method `cartesian_product` which can be called
//! on *any* iterator, such as the one produced by `HashSet::into_iter`. This method
//! returns a structure that implements the `Iterator` trait, allowing one to call
//! methods like `collect`.
//!
//! The snippet above is provided as a unit test, which you can run with
//! `cargo test product`. The test will not compile until you build the API.
//!
//! To get you started, I would read Rust's documentation on how to implement an iterator:
//! https://doc.rust-lang.org/std/iter/index.html#implementing-iterator

trait Cartesian<T: Clone> {
    fn cartesian_product<U: Clone>(
        self: Self,
        iterator: impl Iterator<Item = U>,
    ) -> CartesianProduct<T, U>;
}

struct CartesianProduct<T, U> {
    first: Vec<T>,
    second: Vec<U>,
    i1: usize,
    i2: usize,
}

impl<T, U> CartesianProduct<T, U> {
    fn new(
        iterator1: impl Iterator<Item = T>,
        iterator2: impl Iterator<Item = U>,
    ) -> CartesianProduct<T, U> {
        let first: Vec<T> = iterator1.collect();
        let second: Vec<U> = iterator2.collect();
        return CartesianProduct {
            first: first,
            second: second,
            i1: 0,
            i2: 0,
        };
    }
}

impl<T: Clone, U: Clone> Iterator for CartesianProduct<T, U> {
    type Item = (T, U);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i1 == self.first.len() || self.second.len() == 0 {
            return None;
        }
        let pair = (self.first[self.i1].clone(), self.second[self.i2].clone());

        if self.i2 == self.second.len() - 1 {
            self.i2 = 0;
            self.i1 += 1;
        } else {
            self.i2 += 1;
        }

        Some(pair)
    }
}

impl<S: Iterator<Item = T>, T: Clone> Cartesian<T> for S {
    fn cartesian_product<U: Clone>(
        self,
        iterator: impl Iterator<Item = U>,
    ) -> CartesianProduct<T, U> {
        return CartesianProduct::new(self, iterator);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn cartesian_product_test() {
        let h1 = hashset![1, 2];
        let h2 = hashset![3, 4];
        let product = h1.into_iter().cartesian_product(h2.into_iter());
        assert_eq!(
            product.collect::<HashSet<_>>(),
            hashset![(1, 3), (1, 4), (2, 3), (2, 4)]
        )
    }
}

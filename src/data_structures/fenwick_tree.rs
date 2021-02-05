// most source from https://github.com/rust-bio/rust-bio/blob/master/src/data_structures/bit_tree.rs
use crate::data_structures::monoid::Monoid;

/// In a max bit tree or Fenwick Tree, get(i) will return the largest element e that has been added
/// to the bit tree with set(j, e), where j <= i. Initially all positions have
/// the value T::default(). Note that a set cannot be 'undone' by inserting
/// a smaller element at the same index.
/// Time Complexity: O(n) to build a new tree or O(log n) for get() and set() operations,
/// where `n = tree.len()`.
#[derive(Debug, Clone)]
pub struct FenwickTree<T> {
    tree: Vec<T>,
    len: usize,
}

impl<T: Default + Clone + Monoid> FenwickTree<T> {
    pub fn new(len: usize) -> Self {
        Self {
            tree: vec![T::default(); len + 1],
            len: len + 1,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns the largest element e that has been added
    /// to the bit tree with set(j, e), where j <= i.
    pub fn get(&self, idx: usize) -> T {
        let mut idx = idx + 1;
        let mut sum = T::default();

        while idx > 0 {
            sum = T::mappend(&sum, &self.tree[idx]);
            idx -= (idx as isize & -(idx as isize)) as usize;
        }
        sum
    }

    /// Set the value `val` at position `idx`; `val` will
    /// be returned for any get(j) where j >= idx, if
    /// it is the maximum value inserted between 0 and j.
    /// Inserting a value val2 after inserting val1 where val1 > val2
    /// will have no effect.
    pub fn set(&mut self, idx: usize, val: T) {
        let mut idx = idx + 1;
        while idx < self.len() {
            self.tree[idx] = T::mappend(&self.tree[idx], &val);
            idx += (idx as isize & -(idx as isize)) as usize;
        }
    }
}

#[cfg(test)]
mod test_bit_tree {
    use super::*;
    use crate::data_structures::monoid::Max;

    #[test]
    pub fn test_bit_tree() {
        let mut bit = FenwickTree::<Max<(i64, i64)>>::new(10);

        bit.set(0, Max::from((1, 0)));
        bit.set(1, Max::from((1, 1)));
        bit.set(2, Max::from((2, 2)));
        bit.set(3, Max::from((3, 3)));
        bit.set(4, Max::from((2, 4)));
        bit.set(5, Max::from((2, 5)));
        bit.set(6, Max::from((4, 6)));
        bit.set(7, Max::from((5, 7)));

        assert_eq!(bit.get(0), Max::from((1, 0)));
        assert_eq!(bit.get(1), Max::from((1, 1))); 
        assert_eq!(bit.get(2), Max::from((2, 2))); 
        assert_eq!(bit.get(3), Max::from((3, 3))); 
        assert_eq!(bit.get(4), Max::from((3, 3))); 
        assert_eq!(bit.get(5), Max::from((3, 3))); 
        assert_eq!(bit.get(6), Max::from((4, 6))); 
        assert_eq!(bit.get(7), Max::from((5, 7))); 
    }
}
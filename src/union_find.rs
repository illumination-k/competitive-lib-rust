// from petgraph to customize UnionFind. rank -> size

use num_traits::PrimInt;

#[derive(Debug, Clone)]
pub struct UnionFind<K> {
    // For element at index *i*, store the index of its parent; the representative itself
    // stores its own index. This forms equivalence classes which are the disjoint sets, each
    // with a unique representative.
    parent: Vec<K>,

    // size
    size: Vec<usize>,
}

#[inline]
unsafe fn get_unchecked<K>(xs: &[K], index: usize) -> &K {
    debug_assert!(index < xs.len());
    xs.get_unchecked(index)
}

#[inline]
unsafe fn get_unchecked_mut<K>(xs: &mut [K], index: usize) -> &mut K {
    debug_assert!(index < xs.len());
    xs.get_unchecked_mut(index)
}

impl<K> UnionFind<K>
where
    K: PrimInt,
{
    /// Create a new `UnionFind` of `n` disjoint sets.
    pub fn new(n: usize) -> Self {
        let size = vec![1; n];
        let parent = (0..n).map(|x| K::from(x).unwrap()).collect::<Vec<K>>();

        UnionFind { parent, size }
    }

    /// Return the representative for `x`.
    ///
    /// **Panics** if `x` is out of bounds.
    pub fn find(&self, x: K) -> K {
        assert!(x.to_usize().unwrap() < self.parent.len());
        unsafe {
            let mut x = x;
            loop {
                // Use unchecked indexing because we can trust the internal set ids.
                let xparent = *get_unchecked(&self.parent, x.to_usize().unwrap());
                if xparent == x {
                    break;
                }
                x = xparent;
            }
            x
        }
    }

    /// Return the representative for `x`.
    ///
    /// Write back the found representative, flattening the internal
    /// datastructure in the process and quicken future lookups.
    ///
    /// **Panics** if `x` is out of bounds.
    pub fn find_mut(&mut self, x: K) -> K {
        assert!(x.to_usize().unwrap() < self.parent.len());
        unsafe { self.find_mut_recursive(x) }
    }

    unsafe fn find_mut_recursive(&mut self, mut x: K) -> K {
        let mut parent = *get_unchecked(&self.parent, x.to_usize().unwrap());
        while parent != x {
            let grandparent = *get_unchecked(&self.parent, parent.to_usize().unwrap());
            *get_unchecked_mut(&mut self.parent, x.to_usize().unwrap()) = grandparent;
            x = parent;
            parent = grandparent;
        }
        x
    }

    /// Returns `true` if the given elements belong to the same set, and returns
    /// `false` otherwise.
    pub fn equiv(&self, x: K, y: K) -> bool {
        self.find(x) == self.find(y)
    }

    /// Unify the two sets containing `x` and `y`.
    ///
    /// Return `false` if the sets were already the same, `true` if they were unified.
    ///
    /// **Panics** if `x` or `y` is out of bounds.
    pub fn union(&mut self, x: K, y: K) -> bool {
        if x == y {
            return false;
        }
        let xrep = self.find_mut(x);
        let yrep = self.find_mut(y);

        if xrep == yrep {
            return false;
        }

        let xrepu = xrep.to_usize().unwrap();
        let yrepu = yrep.to_usize().unwrap();
        let xsize = self.size[xrepu];
        let ysize = self.size[yrepu];

        // The rank corresponds roughly to the depth of the treeset, so put the
        // smaller set below the larger
        if xsize > ysize {
            self.parent[yrepu] = xrep;
            self.size[xrepu] += ysize;
        } else {
            self.parent[xrepu] = yrep;
            self.size[yrepu] += xsize;
        }
        
        true
    }

    /// Return a vector mapping each element to its representative.
    pub fn into_labeling(mut self) -> Vec<K> {
        // write in the labeling of each element
        unsafe {
            for ix in 0..self.parent.len() {
                let k = *get_unchecked(&self.parent, ix);
                let xrep = self.find_mut_recursive(k);
                *self.parent.get_unchecked_mut(ix) = xrep;
            }
        }
        self.parent
    }

    pub fn size(&self, x: K) -> usize {
        let xrep = self.find(x);
        let xrepu = xrep.to_usize().unwrap();

        self.size[xrepu]
    }
}


#[cfg(test)]
mod test {

}

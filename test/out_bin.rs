use num_traits::*;

use competitive_internal_mod::data_structures::segment_tree;
use competitive_internal_mod::math::*;

mod competitive_internal_mod {
    pub mod math {
        use num_traits::{one, zero, NumCast, PrimInt};

        pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
            if b == zero() {
                a
            } else {
                gcd(b, a % b)
            }
        }

        pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
            a / gcd(a, b) * b
        }

        pub fn gcd_list<T: PrimInt>(vec: Vec<T>) -> T {
            assert!(vec.len() > 1);
            vec.iter().fold(vec[0], |acc, x| gcd(*x, acc))
        }

        pub fn lcm_list<T: PrimInt>(vec: Vec<T>) -> T {
            assert!(vec.len() > 1);
            vec.iter().fold(vec[0], |acc, x| lcm(*x, acc))
        }

        pub fn quadratic_formula<T: NumCast>(a: T, b: T, c: T) -> Option<(f64, f64)> {
            let a = a.to_f64().unwrap();
            let b = b.to_f64().unwrap();
            let c = c.to_f64().unwrap();

            let descriminant = b * b - 4.0 * a * c;

            if descriminant > 0.0 {
                let ans_1 = (-b + descriminant.sqrt()) / (2.0 * a);
                let ans_2 = (-b - descriminant.sqrt()) / (2.0 * a);
                return Some((ans_1, ans_2));
            } else if descriminant == 0.0 {
                let ans = -b / (2.0 * a);
                return Some((ans, ans));
            } else {
                return None;
            }
        }

        fn safe_mod(mut x: i64, modulo: i64) -> i64 {
            x %= modulo;
            if x < 0 {
                x += modulo;
            }
            x
        }

        pub fn ext_gcd<T: NumCast + PrimInt>(a: T, b: T) -> (T, T) {
            let a = a.to_i64().expect("a can not convert to i64");
            let b = b.to_i64().expect("b cannot convert to i64");
            let a = safe_mod(a, b);
            if a == 0 {
                return (T::from(b).unwrap(), T::from(0).unwrap());
            }

            // Contracts:
            // [1] s - m0 * a = 0 (mod b)
            // [2] t - m1 * a = 0 (mod b)
            // [3] s * |m1| + t * |m0| <= b
            let mut s = b;
            let mut t = a;
            let mut m0 = 0;
            let mut m1 = 1;

            while t != 0 {
                let u = s / t;
                s -= t * u;
                m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

                // [3]:
                // (s - t * u) * |m1| + t * |m0 - m1 * u|
                // <= s * |m1| - t * u * |m1| + t * (|m0| + |m1| * u)
                // = s * |m1| + t * |m0| <= b

                std::mem::swap(&mut s, &mut t);
                std::mem::swap(&mut m0, &mut m1);
            }
            // by [3]: |m0| <= b/g
            // by g != b: |m0| < b/g
            if m0 < 0 {
                m0 += b / s;
            }
            (T::from(s).unwrap(), T::from(m0).unwrap())
        }

        pub fn inv_mod<T: NumCast + PrimInt>(x: T, m: T) -> T {
            assert!(one::<T>() <= m);
            let z = ext_gcd(x, m);
            assert!(z.0 == one::<T>());
            z.1
        }

        /// a0: the first term of serires
        /// d: common difference
        /// n: number of terms
        pub fn arithmetic_progression<T: PrimInt>(a0: T, d: T, n: T) -> T {
            n * ((T::one() + T::one()) * a0 + (n - T::one()) * d) / (T::one() + T::one())
        }
    }
    pub mod data_structures {
        pub mod segment_tree {
            // https://github.com/tanakh/competitive-rs/blob/master/src/segment_tree.rs

            pub mod monoid {
                use num_traits::{Bounded, One, Zero};
                use std::ops::*;

                pub trait Monoid: Sized {
                    /// 単位元
                    fn mempty() -> Self;

                    /// op
                    fn mappend(l: &Self, r: &Self) -> Self;
                }
                #[derive(Debug, Clone, Copy, Default)]
                pub struct Sum<T>(pub T);

                impl<T: Copy + Zero + Add<Output = T>> Monoid for Sum<T> {
                    fn mempty() -> Self {
                        Self(T::zero())
                    }

                    fn mappend(l: &Self, r: &Self) -> Self {
                        Self(l.0 + r.0)
                    }
                }

                impl<T> From<T> for Sum<T> {
                    fn from(v: T) -> Self {
                        Sum(v)
                    }
                }

                #[derive(Clone, Copy, Debug, Default)]
                pub struct Product<T>(pub T);

                impl<T: Copy + One + Mul<Output = T>> Monoid for Product<T> {
                    fn mempty() -> Self {
                        Self(T::one())
                    }

                    fn mappend(l: &Self, r: &Self) -> Self {
                        Self(l.0 * r.0)
                    }
                }

                impl<T> From<T> for Product<T> {
                    fn from(v: T) -> Self {
                        Product(v)
                    }
                }

                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct Max<T>(pub T);

                impl<T: Copy + Ord + Bounded> Monoid for Max<T> {
                    fn mempty() -> Self {
                        Self(<T as Bounded>::min_value())
                    }

                    fn mappend(l: &Self, r: &Self) -> Self {
                        Self(l.0.max(r.0))
                    }
                }

                impl<T> From<T> for Max<T> {
                    fn from(v: T) -> Self {
                        Max(v)
                    }
                }

                #[derive(Clone, Copy, Debug, Default)]
                pub struct Min<T>(pub T);

                impl<T: Copy + Ord + Bounded> Monoid for Min<T> {
                    fn mempty() -> Self {
                        Self(<T as Bounded>::max_value())
                    }

                    fn mappend(l: &Self, r: &Self) -> Self {
                        Self(l.0.min(r.0))
                    }
                }

                impl<T> From<T> for Min<T> {
                    fn from(v: T) -> Self {
                        Min(v)
                    }
                }

                #[derive(Debug, Clone, Copy, Default)]
                pub struct XOR<T>(pub T);

                impl<T: Copy + Zero + BitXor<Output = T>> Monoid for XOR<T> {
                    fn mempty() -> Self {
                        Self(T::zero())
                    }

                    fn mappend(l: &Self, r: &Self) -> Self {
                        Self(l.0 ^ r.0)
                    }
                }

                impl<T> From<T> for XOR<T> {
                    fn from(v: T) -> Self {
                        XOR(v)
                    }
                }
            }
            use monoid::*;

            use std::{ops::Bound, ops::RangeBounds};

            /// Segment tree
            #[derive(Debug)]
            pub struct SegmentTree<T> {
                len: usize,
                v: Vec<T>,
            }

            impl<T: Clone + Monoid> SegmentTree<T> {
                /// O(n).
                /// Construct segment tree for given size.
                pub fn new(n: usize) -> Self {
                    let s: &[T] = &[];
                    Self::init(n, s)
                }

                /// O(n).
                /// Construct segment tree from slice.
                pub fn from_slice(s: &[impl Into<T> + Clone]) -> Self {
                    Self::init(s.len(), s)
                }

                fn init(len: usize, s: &[impl Into<T> + Clone]) -> Self {
                    let n = len.next_power_of_two();
                    let mut v = vec![T::mempty(); n * 2 - 1];
                    for i in 0..s.len() {
                        v[n - 1 + i] = s[i].clone().into();
                    }

                    let mut l = n / 2;
                    let mut ofs = n - 1 - l;

                    while l > 0 {
                        for i in 0..l {
                            let ix = ofs + i;
                            v[ix] = T::mappend(&v[ix * 2 + 1], &v[ix * 2 + 2]);
                        }
                        l /= 2;
                        ofs -= l;
                    }

                    Self { len, v }
                }

                /// O(1).
                /// Length of sequence.
                pub fn len(&self) -> usize {
                    self.len
                }

                /// O(log n).
                /// Set v to `i`-th element.
                /// `s[i] = v`
                pub fn set(&mut self, i: usize, v: impl Into<T>) {
                    let n = (self.v.len() + 1) / 2;
                    let mut cur = n - 1 + i;
                    self.v[cur] = v.into();
                    while cur > 0 {
                        cur = (cur - 1) / 2;
                        self.v[cur] = T::mappend(&self.v[cur * 2 + 1], &self.v[cur * 2 + 2]);
                    }
                }

                /// O(log n).
                /// mappend v to `i`-th element
                /// `s[i] = mappend(s[i], v)`
                pub fn mappend(&mut self, i: usize, v: impl Into<T>) {
                    self.set(i, T::mappend(&self.get(i), &v.into()));
                }

                /// O(1).
                /// Get i-th element
                /// Equals to `query(i, i + 1)`
                pub fn get(&self, i: usize) -> T {
                    let n = (self.v.len() + 1) / 2;
                    self.v[n - 1 + i].clone()
                }

                /// O(log n).
                /// Query for `range`.
                /// Returns `T::mconcat(&s[range])`.
                ///
                /// # Examples
                ///
                /// ```
                /// use competitive::data_structures::monoid::Sum;
                /// use competitive::data_structures::segment_tree::SegmentTree;
                ///
                /// let mut st = SegmentTree::<Sum<i64>>::new(5);
                /// st.set(2, 3);
                /// assert_eq!(st.query(0..=2).0, 3);
                /// assert_eq!(st.query(0..2).0, 0);
                /// ```
                ///
                pub fn query(&self, range: impl RangeBounds<usize>) -> T {
                    let l = match range.start_bound() {
                        Bound::Included(v) => *v,
                        Bound::Excluded(v) => v + 1,
                        Bound::Unbounded => 0,
                    };
                    let r = match range.end_bound() {
                        Bound::Included(v) => v + 1,
                        Bound::Excluded(v) => *v,
                        Bound::Unbounded => self.len,
                    };

                    assert!(l <= r);
                    assert!(r <= self.len);

                    let n = (self.v.len() + 1) / 2;
                    let mut l = n + l;
                    let mut r = n + r;

                    let mut ret_l = T::mempty();
                    let mut ret_r = T::mempty();
                    while l < r {
                        if l & 1 != 0 {
                            ret_l = T::mappend(&ret_l, &self.v[l - 1]);
                            l += 1;
                        }
                        if r & 1 != 0 {
                            r -= 1;
                            ret_r = T::mappend(&self.v[r - 1], &ret_r);
                        }
                        l /= 2;
                        r /= 2;
                    }

                    T::mappend(&ret_l, &ret_r)
                }
            }
        }
    }
}


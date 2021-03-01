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

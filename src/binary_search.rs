use std::ops::{Add, Div};

pub fn binary_search<T>(left: T, right: T, f: impl Fn(T) -> bool) -> T
where
    T: Copy + Add<Output = T> + Div<Output = T> + PartialEq + From<u8>,
{
    let two = T::from(2u8);

    let mut l = left;
    let mut r = right;

    loop {
        let mid: T = (l + r) / two;

        if l == mid || r == mid {
            break l;
        }

        if f(mid) {
            l = mid;
        } else {
            r = mid;
        }
    }
}

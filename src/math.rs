use std::fmt::Debug;

use num_traits::{one, zero, NumCast, PrimInt};

/// GCD
pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    if b == zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// LCM
pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

/// GCD of vec
pub fn gcd_list<T: PrimInt>(vec: Vec<T>) -> T {
    assert!(vec.len() > 1);
    vec.iter().fold(vec[0], |acc, x| gcd(*x, acc))
}

/// LCM of vec
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

/// sum of Arithmetic progression  
/// **a0**: the first term of serires    
/// **d**: common difference  
/// **n**: number of terms  
pub fn arithmetic_progression_sum<T: PrimInt>(a0: T, d: T, n: T) -> T {
    n * ((T::one() + T::one()) * a0 + (n - T::one()) * d) / (T::one() + T::one())
}

/// sum of geometric progression  
/// **a0**: the first term of serires    
/// **r**: geometric progression    
/// **n**: number of terms 
pub fn geometric_progression_sum<T: PrimInt + Debug>(a0: T, r: T, n: T) -> T {
    assert_ne!(r, T::one());

    a0 * (T::one() - r.pow(n.to_u32().unwrap())) / (T::one() - r)
}

#[cfg(test)]
mod test {
    use super::quadratic_formula;
    use super::{gcd, gcd_list, lcm, lcm_list};
    #[test]
    fn gcd_1() {
        assert_eq!(gcd(10, 2), 2);
        assert_eq!(gcd(12, 42), 6);
    }

    #[test]
    fn gcd_list_1() {
        assert_eq!(gcd_list(vec![12, 42, 72]), 6);
    }

    #[test]
    fn lcm_1() {
        assert_eq!(lcm(12, 42), 84);
    }

    #[test]
    fn lcm_list_1() {
        assert_eq!(lcm_list(vec![12, 42, 72]), 504);
    }

    #[test]
    fn test_quadratic_formula_1() {
        assert_eq!(quadratic_formula(4, 4, 1), Some((-0.5, -0.5)));
        assert_eq!(quadratic_formula(1, -6, 8), Some((4.0, 2.0)));
        assert_eq!(quadratic_formula(1, 2, 3), None);
    }
}

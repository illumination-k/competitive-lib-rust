use num_traits::{ PrimInt, zero, NumCast };
use cargo_snippet::snippet;

#[snippet("gcd")]
pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    if b == zero()  {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet("lcm")]
#[snippet(include="gcd")]
pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

#[snippet("gcd_list")]
#[snippet(include="gcd")]
pub fn gcd_list<T: PrimInt>(vec: Vec<T>) -> T {
    assert!(vec.len() > 1);
    vec.iter().fold(vec[0], |acc, x| gcd(*x, acc))
}

#[snippet("lcm_list")]
#[snippet(include="lcm")]
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
        if ans_1 > ans_2 {
            return Some((ans_1, ans_2))
        } else {
            return Some((ans_2, ans_1))
        }
    } else if descriminant == 0.0 {
        let ans = -b / (2.0 * a);
        return Some((ans, ans));
    } else {
        return None;
    }
}


#[cfg(test)]
mod test {
    use super::gcd;
    use super::quadratic_formula;
    #[test]
    fn gcd_1() {
        assert_eq!(gcd(10, 2), 2);
    }

    #[test]
    fn test_quadratic_formula_1() {
        assert_eq!(quadratic_formula(4, 4, 1), Some((-0.5, -0.5)));
        assert_eq!(quadratic_formula(1, -6, 8), Some((4.0, 2.0)));
        assert_eq!(quadratic_formula(1, 2, 3), None);
    }
}
use num_traits::{ PrimInt, zero };
use cargo_snippet::snippet;

pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    if b == zero()  {
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


#[cfg(test)]
mod test {
    use super::gcd;

    #[test]
    fn gcd_1() {
        assert_eq!(gcd(10, 2), 2);
    }
}
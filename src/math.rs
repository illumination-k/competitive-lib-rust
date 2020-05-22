use cargo_snippet::snippet;


pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}


pub fn gcd_list(vec: Vec<usize>) -> usize {
    assert!(vec.len() > 1);
    vec.iter().fold(vec[0], |acc, x| gcd(*x, acc))
}

pub fn lcm_list(vec: Vec<usize>) -> usize {
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
use cargo_snippet::snippet;

#[snippet("is_prime")]
pub fn is_prime(n: usize) -> bool {
    let mut flag: bool = true;

    if n == 1 { flag = false }
    let mut i: usize = 2;
    while i * i <= n {
        if n % i == 0 {
            flag = false;
            break;
        }
        i += 1;
    }
    flag
}

#[snippet("enum_divisors")]
pub fn enum_divisors(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();

    let mut i: usize = 1;

    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if n / i != i { res.push(n / i) }
        }
        i += 1;
    }
    res.sort();
    res
}


#[snippet("prime_factorize")]
pub fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();

    let mut i: usize = 2;

    while i * i <= n {
        if n % i == 0 {
            let mut ex: usize = 0;

            while n % i == 0 {
                ex += 1;
                n = n / i;
            }
            res.push((i, ex));
        }
        i += 1;
    }

    if n != 1 {res.push((n, 1))}

    res
}


#[cfg(test)]
mod test {
    use super::is_prime;
    use super::enum_divisors;
    use super::prime_factorize;

    #[test]
    fn test_is_prime_2() {
        let flag = is_prime(2);

        assert_eq!(flag, true);
    }

    #[test]
    fn test_is_prime_13() {
        let flag = is_prime(13);
        assert_eq!(flag, true);
    }

    #[test]
    fn test_is_prime_20() {
        let flag = is_prime(20);
        assert_eq!(flag, false);
    }

    #[test]
    fn test_enum_dividors_12() {
        let ans = enum_divisors(12);
        assert_eq!(ans, vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_prime_factorize_2020() {
        let vec = prime_factorize(2020);
        assert_eq!(vec, vec![(2, 2), (5, 1), (101, 1)]);
    }
}
use cargo_snippet::snippet;
use num_traits::*;

#[snippet("is_prime")]
pub fn is_prime<T: PrimInt+NumAssign>(n: T) -> bool {
    // O(sqrt(n))
    let mut flag: bool = true;

    if n == one() { flag = false }
    let mut i: T = one::<T>().signed_shl(1);
    while i * i <= n {
        if n % i == zero() {
            flag = false;
            break;
        }
        i += one();
    }
    flag
}

#[snippet("enum_divisors")]
pub fn enum_divisors<T: PrimInt+NumAssign>(n: T) -> Vec<T> {
    // O(sqrt(n))
    let mut res: Vec<T> = Vec::new();

    let mut i: T = one();

    while i * i <= n {
        if n % i == zero() {
            res.push(i);
            if n / i != i { res.push(n / i) }
        }
        i += one();
    }
    res.sort();
    res
}


#[snippet("prime_factorize")]
pub fn prime_factorize<T: PrimInt+NumAssign>(mut n: T) -> Vec<(T, T)> {
    // O(sqrt(n))
    let mut res: Vec<(T, T)> = Vec::new();

    let mut i: T = one::<T>().signed_shl(1);

    while i * i <= n {
        if n % i == zero() {
            let mut ex = zero::<T>();

            while n % i == zero() {
                ex += one();
                n = n / i;
            }
            res.push((i, ex));
        }
        i += one();
    }

    if n != one() {res.push((n, one()))}

    res
}


#[snippet("sieve_of_eratosthenes")]
pub fn sieve_of_eratosthenes<T: NumCast>(n: T) -> Vec<usize> {
    let n = n.to_usize().expect("cannot convert n to usize");
    if n < 2 {
        return vec![]
    }

    let mut flags = vec![true; n / 2];
    flags[0] = false;

    let sqrt_x = (((n as f64).sqrt() + 0.1).ceil() + 0.5) as usize;
    let sqrt_xi = sqrt_x / 2;

    for i in 1..sqrt_xi {
        if !flags[i] { continue; }
        let p = 2 * i + 1;
        let start = 2 * i * (i + 1);
        for mult in (start..flags.len()).step_by(p) {
            flags[mult] = false;
        }
    }

    std::iter::once(2).chain(
        flags.iter().enumerate()
            .filter(|(_i, flag)| **flag)
            .map(|(i, _flag)| 2 * i + 1)
    ).collect()
}

#[snippet("osa_k")]
#[snippet(include="osa_k_make_sieves")]
#[snippet(include="osa_k_impl")]
#[derive(Debug, Clone)]
pub struct OsaK<T: PrimInt + std::hash::Hash + NumAssign > {
    sieve: Vec<T>
}

#[snippet("osa_k_make_sieves")]
fn _make_sieve<T: PrimInt>(mut maxu: usize) -> Vec<T> {
    maxu += 1;
    let mut sieve: Vec<usize> = (0..maxu).collect();

    let mut i = 2;
    while i*i < maxu {
        if sieve[i] < i { i += 1; continue; }
        for j in (i*i..maxu).step_by(i) {
            if sieve[j] == j { sieve[j] = i }
        }
        i += 1;
    }

    sieve.into_iter()
        .filter_map(|x| T::from(x))
        .collect()
}

#[snippet("osa_k_impl")]
impl<T: PrimInt + std::hash::Hash + NumAssign > OsaK<T> {
    /// O(maxloglog(max))
    /// construct osa-k from max size
    pub fn new(max: T) -> Self {
        let maxu = max.to_usize().expect("cannot convert to usize");
        let sieve = _make_sieve(maxu);

        Self { sieve }
    }

    /// O(max(vec)loglog(max(vec)))
    /// construct osa-k from Vector
    pub fn from(vec: Vec<T>) -> Self {
        assert!(vec.len() > 0);
        let max = vec.iter().max().unwrap();
        let maxu = max.to_usize().unwrap();
        let sieve = _make_sieve(maxu);
        
        Self { sieve }
    }

    /// O(1)
    /// test x is prime or not
    pub fn is_prime(&self, x: T) -> bool {
        if x == one() || x == zero() { return false }
        self.sieve[x.to_usize().unwrap()] == x
    } 

    /// O(log(n))
    pub fn prime_factorize(&self, mut n: T) -> std::collections::HashMap<T, T> {
        let mut res: std::collections::HashMap<T, T> = std::collections::HashMap::new();
        while n > one() {
            *res.entry(self.sieve[n.to_usize().unwrap()]).or_insert(zero()) += one();
            n /= self.sieve[n.to_usize().unwrap()]
        }
        res
    }
}


#[cfg(test)]
mod test {
    use super::is_prime;
    use super::enum_divisors;
    use super::prime_factorize;
    use super::sieve_of_eratosthenes;
    use super::OsaK;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(13));
        assert!((!is_prime(20)))
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

    #[test]
    fn test_eratosthenes_1() {
        let vec1 = sieve_of_eratosthenes(1);
        assert_eq!(vec1, vec![]);

        let vec2 = sieve_of_eratosthenes(2);
        assert_eq!(vec2, vec![2]);

        let vec20 = sieve_of_eratosthenes(20);
        assert_eq!(vec20, vec![2, 3, 5, 7, 11, 13, 17, 19]);

        let vec100 = sieve_of_eratosthenes(100);
        assert_eq!(vec100, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97])
    }

    #[test]
    fn test_osa_k() {
        let osa_k = OsaK::new(18);

        // prime
        assert!(osa_k.is_prime(2));
        assert!(osa_k.is_prime(5));
        assert!(osa_k.is_prime(7));
        
        // not prime
        assert!(!osa_k.is_prime(0));
        assert!(!osa_k.is_prime(1));
        assert!(!osa_k.is_prime(12));
        assert!(!osa_k.is_prime(14));

        // bound
        assert!(!osa_k.is_prime(18));

        // !TODO
        // factors
    }
}